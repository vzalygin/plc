pub mod asm;


use asm::Asm;
use crate::common::{Ast, Term};

const START_LABEL: &str = "_start";
const OP_STACK_LABEL: &str = "op_stack";
const OP_STACK_BASE_LABEL: &str = "op_stack_base";
const OP_SIZE: Operand = Operand::Dword;
const OP_SIZE_BYTES: i64 = 4;
const OP_STACK_SIZE: i64 = OP_SIZE_BYTES * 1024;

pub fn translate(ast: &Ast) -> Asm {
    let asm = prelude();

    let asm = ast.terms
        .iter()
        .fold(asm, |asm, term| {
            asm.append(translate_term(term))
        }
    );

    let asm = asm.append(epilogue());

    asm
}

fn prelude() -> Asm {
    let data = vec![
        i!(section!(Data)),
        i!(label!(OP_STACK_LABEL), dd!(Db), Op::Literal(OP_STACK_SIZE)),
        i!(label!(OP_STACK_BASE_LABEL), dd!(Equ), opexpr!(format!("$ - {OP_STACK_LABEL}")))
    ];
    let bss = vec![
        i!(section!(Bss)),
    ];
    let text = vec![
        i!(section!(Text)),
        i!(Global, oplabel!(START_LABEL)),
        i!(label!(START_LABEL)),
        i!(Mov, reg!(Rbx), oplabel!(OP_STACK_BASE_LABEL)),
    ];

    Asm::from_instructions(data, bss, text)
}

fn epilogue() -> Asm {
    Asm::from_text(vec![
        i!(Mov, reg!(Rdx), Op::Literal(0)), // success exit code
        i!(Mov, reg!(Rax), Op::Literal(60)), // exit call
        i!(Syscall),
    ])
}

fn translate_term(term: &Term) -> Asm {
    match term {
        Term::Int(number) => Asm::from_text(vec![
            i!(Sub, reg!(Rbx), Op::Literal(OP_SIZE_BYTES)),
            i!(Mov, OP_SIZE, indirect_register!(Rbx), Op::Literal(*number as i64)),
        ]),
        Term::Add => Asm::from_text(vec![
            i!(Mov, reg!(Eax), indirect_register!(Rbx)),
            i!(Add, opexpr!(format!("[rbx+{OP_SIZE_BYTES}]")), reg!(Edi)),
            i!(Add, reg!(Rbx), Op::Literal(OP_SIZE_BYTES)),
        ]),
        Term::Sub => Asm::from_text(vec![
            i!(Mov, reg!(Eax), indirect_register!(Rbx)),
            i!(Sub, opexpr!(format!("[rbx+{OP_SIZE_BYTES}]")), reg!(Edi)),
            i!(Add, reg!(Rbx), Op::Literal(OP_SIZE_BYTES)),
        ]),
        Term::Mul => Asm::from_text(vec![
            i!(Mov, reg!(Eax), indirect_register!(Rbx)),
            i!(Cdq),
        ]),
        Term::Div => Asm::from_text(vec![
            
        ]),
        Term::Print => Asm::from_text(vec![
        ]),
    }
}
