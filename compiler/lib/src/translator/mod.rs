mod asm;
mod consts;
mod stdlib;
mod util;

pub use {
    asm::Asm,
    stdlib::{make_std_lib, STD_PRINT_FN_LABEL},
    util::LabelGenerator,
};

use crate::common::{Ast, Term};
use consts::*;
use stdlib::STD_EXIT_FN_LABEL;
use x64asm::{indirect_register, macros::*};

pub fn translate(ast: &Ast) -> Asm {
    let mut label_generator = LabelGenerator::default();
    let asm = prelude();

    let asm = ast.terms.iter().fold(asm, |asm, term| {
        asm.append(translate_term(term, &mut label_generator))
    });

    asm.append(epilogue())
}

fn prelude() -> Asm {
    let rodata = vec![i!(section!(Rodata))];
    let bss = vec![
        i!(section!(Bss)),
        i!(
            label!(OP_STACK_LABEL),
            opexpr!(format!("resb {OP_STACK_SIZE}"))
        ),
        i!(label!(OP_STACK_BASE_LABEL), opexpr!(format!("resd 1"))),
    ];
    let text = vec![
        i!(Extern, oplabel!(STD_PRINT_FN_LABEL.to_string())),
        i!(Extern, oplabel!(STD_EXIT_FN_LABEL.to_string())),
        i!(section!(Text)),
        i!(Global, oplabel!(START_LABEL)),
        i!(label!(START_LABEL)),
        i!(Mov, reg!(Ebx), oplabel!(OP_STACK_LABEL)),
        i!(Add, reg!(Ebx), Op::Literal(OP_STACK_SIZE)),
        i!(Mov, opexpr!(format!("[{OP_STACK_BASE_LABEL}]")), reg!(Ebx)),
    ];

    Asm::new(rodata, bss, text, vec![])
}

fn epilogue() -> Asm {
    Asm::empty().text([
        i!(Mov, reg!(Rdi), Op::Literal(0)),
        i!(Call, oplabel!(STD_EXIT_FN_LABEL.to_string())),
    ])
}

fn translate_term(term: &Term, label_generator: &mut LabelGenerator) -> Asm {
    match term {
        Term::Int(number) => Asm::empty().text([
            i!(Sub, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
            i!(
                Mov,
                indirect_register!(Ebx),
                OP_SIZE,
                Op::Literal(*number as i64)
            ),
        ]),
        Term::Add => Asm::empty().text([
            i!(Mov, reg!(Eax), indirect_register!(Ebx)),
            i!(Add, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
            i!(Add, indirect_register!(Ebx), reg!(Eax)),
        ]),
        Term::Sub => Asm::empty().text([
            i!(Mov, reg!(Eax), indirect_register!(Ebx)),
            i!(Add, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
            i!(Sub, indirect_register!(Ebx), reg!(Eax)),
        ]),
        Term::Mul => Asm::empty().text([
            i!(Mov, reg!(Rax), indirect_register!(Ebx)),
            i!(Add, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
            i!(Mul, opexpr!(format!("dword[EBX]"))),
            i!(Mov, indirect_register!(Ebx), reg!(Eax)),
        ]),
        Term::Div => Asm::empty().text([
            i!(Mov, reg!(Edi), indirect_register!(Ebx)),
            i!(Add, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
            i!(Xor, reg!(Rdx), reg!(Rdx)),
            i!(Mov, reg!(Rax), indirect_register!(Ebx)),
            i!(Cltq),
            i!(Cqto),
            i!(Div, reg!(Edi)),
            i!(Mov, indirect_register!(Ebx), reg!(Eax)),
        ]),
        Term::Print => Asm::empty().text([i!(Call, oplabel!(STD_PRINT_FN_LABEL))]),
        Term::Dup => Asm::empty().text([
            i!(Mov, reg!(Eax), indirect_register!(Ebx)),
            i!(Sub, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
            i!(Mov, indirect_register!(Ebx), reg!(Eax)),
        ]),
        Term::Drop => Asm::empty().text([i!(Add, reg!(Ebx), Op::Literal(OP_SIZE_BYTES))]),
        Term::Take => {
            let exch_cycle_label = label_generator.get_label();
            let no_exch_label = label_generator.get_label();
            Asm::empty().text([
                i!(Xor, reg!(Rcx), reg!(Rcx)),
                i!(Mov, reg!(Ecx), indirect_register!(Ebx)),
                i!(Add, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
                i!(Cmp, reg!(Ecx), Op::Literal(0)),
                i!(Jz, opexpr!(no_exch_label)),
                i!(label!(exch_cycle_label.as_str())),
                i!(
                    Mov,
                    reg!(Eax),
                    opexpr!(format!("[EBX+ECX*{OP_SIZE_BYTES}]"))
                ),
                i!(
                    Mov,
                    reg!(Esi),
                    opexpr!(format!("[EBX+ECX*{OP_SIZE_BYTES}-{OP_SIZE_BYTES}]"))
                ),
                i!(
                    Mov,
                    opexpr!(format!("[EBX+ECX*{OP_SIZE_BYTES}]")),
                    reg!(Esi)
                ),
                i!(
                    Mov,
                    opexpr!(format!("[EBX+ECX*{OP_SIZE_BYTES}-{OP_SIZE_BYTES}]")),
                    reg!(Eax)
                ),
                i!(Sub, reg!(Ecx), Op::Literal(1)),
                i!(Jnz, oplabel!(exch_cycle_label)),
                i!(label!(no_exch_label.as_str())),
            ])
        }
        Term::List { terms } => {
            let label = label_generator.get_label();

            let list_asm = Asm::empty().text([
                i!(Sub, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
                i!(
                    Mov,
                    indirect_register!(Ebx),
                    opexpr!(format!("dword {label}"))
                ),
            ]);

            let inner_asm = Asm::empty().text([i!(label!(label.as_str()))]);
            let inner_asm = terms.iter().fold(inner_asm, |asm, term| {
                asm.append(translate_term(term, label_generator))
            });
            let inner_asm = inner_asm.text([i!(Ret)]);
            let inner_asm = Asm::new(inner_asm.rodata, inner_asm.bss, vec![], inner_asm.text_tail)
                .text_tail(inner_asm.text);

            list_asm.append(inner_asm)
        }
        Term::Apply => Asm::empty().text([
            i!(Add, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
            i!(Call, opexpr!(format!("[EBX-{OP_SIZE_BYTES}]"))),
        ]),
    }
}
