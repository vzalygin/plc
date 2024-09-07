mod asm;
mod stdlib;
mod consts;

pub use {
    asm::Asm,
    stdlib::{
        make_std_lib,
        STD_PRINT_FN_LABEL,
    },
};

use consts::*;
use x64asm::{indirect_register, macros::*};
use crate::common::{Ast, Term};

pub fn translate(ast: &Ast) -> Asm {
    let asm = prelude();

    let asm = ast.terms
        .iter()
        .fold(asm, |asm, term| {
            asm.append(translate_term(term))
        }
    );

    asm.append(epilogue())
}

fn prelude() -> Asm {
    let data = [
        i!(section!(Data)),
    ];
    let bss = [
        i!(section!(Bss)),
        i!(label!(OP_STACK_LABEL), opexpr!(format!("resb {OP_STACK_SIZE}"))),
        i!(label!(OP_STACK_BASE_LABEL), opexpr!(format!("resd 1"))),
    ];
    let text = [
        i!(Extern, oplabel!(STD_PRINT_FN_LABEL.to_string())),

        i!(section!(Text)),
        i!(Global, oplabel!(START_LABEL)),
        i!(label!(START_LABEL)),
        i!(Mov, reg!(Ebx), oplabel!(OP_STACK_LABEL)),
        i!(Add, reg!(Ebx), Op::Literal(OP_STACK_SIZE)),
        i!(Mov, opexpr!(format!("[{OP_STACK_BASE_LABEL}]")), reg!(Ebx)),
    ];

    Asm::from_instructions(data, bss, text)
}

fn epilogue() -> Asm {
    Asm::from_text([
        i!(Mov, reg!(Rdx), Op::Literal(0)), // success exit code
        i!(Mov, reg!(Rax), Op::Literal(60)), // exit call
        i!(Syscall),
    ])
}

fn translate_term(term: &Term) -> Asm {
    match term {
        Term::Int(number) => Asm::from_text([
            i!(Sub, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
            i!(Mov, indirect_register!(Ebx), OP_SIZE, Op::Literal(*number as i64)),
        ]),
        Term::Add => Asm::from_text([
            i!(Mov, reg!(Eax), indirect_register!(Ebx)),
            i!(Add, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
            i!(Add, indirect_register!(Ebx), reg!(Eax)),
        ]),
        Term::Sub => Asm::from_text([
            i!(Mov, reg!(Eax), indirect_register!(Ebx)),
            i!(Add, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
            i!(Sub, indirect_register!(Ebx), reg!(Eax)),
        ]),
        Term::Mul => Asm::from_text([
            i!(Xor, reg!(Rax), reg!(Rax)),
            i!(Mov, reg!(Eax), indirect_register!(Ebx)),
            i!(Add, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
            i!(Mul, OP_SIZE, indirect_register!(Ebx)),
            i!(Mov, indirect_register!(Ebx), reg!(Eax)),
        ]),
        Term::Div => Asm::from_text([
            i!(Mov, reg!(Edi), indirect_register!(Ebx)),
            i!(Add, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
            i!(Xor, reg!(Rax), reg!(Rax)),
            i!(Mov, reg!(Eax), indirect_register!(Ebx)),
            i!(Cltq),
            i!(Cqto),
            i!(Div, reg!(Edi)),
            i!(Mov, indirect_register!(Ebx), reg!(Eax)),
        ]),
        Term::Print => Asm::from_text([
            i!(Call, oplabel!(STD_PRINT_FN_LABEL)),
        ]),
    }
}
