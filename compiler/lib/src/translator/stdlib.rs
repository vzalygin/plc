use x64asm::{
    i, indirect_register,
    instruction::Section::{Bss, Text},
    macros::*,
    section,
};

use super::{asm::Asm, OP_SIZE_BYTES};

pub const STD_PRINT_FN_LABEL: &str = "$std_print";
pub const STD_EXIT_FN_LABEL: &str = "$str_exit";

const OUTPUT_TEMPLATE_LABEL: &str = "$otemplate";
const OUTPUT_TEMPLATE_STR: &str = "%d";

const LIBC_PRINTF_LABEL: &str = "printf";
const LIBC_EXIT_LABEL: &str = "exit";

pub fn make_std_lib() -> Asm {
    let rodata = vec![
        i!(section!(Rodata)),
        i!(
            label!(OUTPUT_TEMPLATE_LABEL),
            dd!(Db),
            opstring!(OUTPUT_TEMPLATE_STR.to_string()),
            Op::Literal(10),
            Op::Literal(0)
        ),
    ];
    let bss = vec![i!(section!(Bss))];
    let text = vec![
        i!(Global, oplabel!(STD_PRINT_FN_LABEL.to_string())),
        i!(Global, oplabel!(STD_EXIT_FN_LABEL.to_string())),
        i!(Extern, oplabel!(LIBC_PRINTF_LABEL.to_string())),
        i!(Extern, oplabel!(LIBC_EXIT_LABEL.to_string())),
        i!(section!(Text)),
        i!(label!(STD_PRINT_FN_LABEL)),
        i!(Push, reg!(Rbp)),
        i!(Mov, reg!(Rbp), reg!(Rsp)),
        i!(And, reg!(Rsp), Op::Literal(-16)),
        i!(Mov, reg!(Rdi), oplabel!(OUTPUT_TEMPLATE_LABEL.to_string())),
        i!(Xor, reg!(Rsi), reg!(Rsi)),
        i!(Mov, reg!(Esi), indirect_register!(Ebx)),
        i!(Call, oplabel!(LIBC_PRINTF_LABEL.to_string())),
        i!(Add, reg!(Ebx), Op::Literal(OP_SIZE_BYTES)),
        i!(Mov, reg!(Rsp), reg!(Rbp)),
        i!(Pop, reg!(Rbp)),
        i!(Ret),
        i!(label!(STD_EXIT_FN_LABEL)),
        i!(And, reg!(Rsp), Op::Literal(-16)),
        i!(Call, oplabel!(LIBC_EXIT_LABEL.to_string())),
    ];

    Asm::new(rodata, bss, text)
}
