pub mod asm;
mod stdlib;
mod consts;

use asm::Asm;
use consts::*;
use stdlib::STD_PRINT_FN_LABEL;
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
        i!(label!(OP_STACK_LABEL), dd!(Db), Op::Literal(OP_STACK_SIZE)),
        i!(label!(OP_STACK_BASE_LABEL), dd!(Equ), opexpr!(format!("$ - {OP_STACK_LABEL}")))
    ];
    let bss = [
        i!(section!(Bss)),
    ];
    let text = [
        i!(section!(Text)),
        i!(Global, oplabel!(START_LABEL)),
        i!(label!(START_LABEL)),
        i!(Mov, reg!(Rbx), oplabel!(OP_STACK_BASE_LABEL)),
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
            i!(Sub, reg!(Rbx), Op::Literal(OP_SIZE_BYTES)),
            i!(Mov, OP_SIZE, indirect_register!(Rbx), Op::Literal(*number as i64)),
        ]),
        Term::Add => Asm::from_text([
            i!(Mov, reg!(Eax), indirect_register!(Rbx)),
            i!(Add, reg!(Rbx), Op::Literal(OP_SIZE_BYTES)),
            i!(Add, indirect_register!(Rbx), reg!(Eax)),
        ]),
        Term::Sub => Asm::from_text([
            i!(Mov, reg!(Eax), indirect_register!(Rbx)),
            i!(Add, reg!(Rbx), Op::Literal(OP_SIZE_BYTES)),
            i!(Sub, indirect_register!(Rbx), reg!(Eax)),
        ]),
        Term::Mul => Asm::from_text([
            i!(Xor, reg!(Rax), reg!(Rax)),
            i!(Mov, reg!(Eax), indirect_register!(Rbx)),
            i!(Add, reg!(Rbx), Op::Literal(OP_SIZE_BYTES)),
            i!(Mul, OP_SIZE, indirect_register!(Rbx)),
            i!(Mov, indirect_register!(Rbx), reg!(Eax)),
        ]),
        Term::Div => Asm::from_text([
            i!(Mov, reg!(Edi), indirect_register!(Rbx)),
            i!(Add, reg!(Rbx), Op::Literal(OP_SIZE_BYTES)),
            i!(Xor, reg!(Rax), reg!(Rax)),
            i!(Mov, reg!(Eax), indirect_register!(Rbx)),
            i!(Cltq),
            i!(Cqto),
            i!(Div, reg!(Edi)),
            i!(Mov, indirect_register!(Rbx), reg!(Eax)),
        ]),
        Term::Print => Asm::from_text([
            i!(Call, oplabel!(STD_PRINT_FN_LABEL)),
        ]),
    }
}
