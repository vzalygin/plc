use x64asm::{macros::*, i, instruction::Section::{Bss, Text}, section};

use super::{asm::Asm, OP_SIZE_BYTES};

pub const STD_PRINT_FN_LABEL: &str = "$std_print";

const OUTPUT_TEMPLATE_LABEL: &str = "$otemplate";
const OUTPUT_TEMPLATE_STR: &str = "`%d\n\0`";

const LIBC_PRINTF_LABEL: &str = "printf";

pub fn make_std_lib() -> Asm {
    let data = [
        i!(section!(Rodata)),
        i!(label!(OUTPUT_TEMPLATE_LABEL), dd!(Db), Op::String(OUTPUT_TEMPLATE_STR.to_string()))
    ];
    let bss = [
        i!(section!(Bss)),
    ];
    let text = [
        i!(Global, Op::String(STD_PRINT_FN_LABEL.to_string())),

        i!(Extern, Op::String(LIBC_PRINTF_LABEL.to_string())),

        i!(section!(Text)),
        
        i!(label!(STD_PRINT_FN_LABEL)),
        i!(Mov, Op::Register(Rdi), Op::String(OUTPUT_TEMPLATE_LABEL.to_string())),
        i!(Call, Op::String(LIBC_PRINTF_LABEL.to_string())),
        i!(Add, reg!(Rbx), Op::Literal(OP_SIZE_BYTES)),
        i!(Ret)
    ];

    Asm::from_instructions(data, bss, text)
}
