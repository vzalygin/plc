use x64asm::{macros::*, i, instruction::Section::{Bss, Data, Text}, section};

use super::asm::Asm;

pub const STD_PRINT_FN_LABEL: &str = "print";

fn make_std_lib() -> Asm {
    let data = [
        i!(section!(Data)),
    ];
    let bss = [
        i!(section!(Bss)),
    ];
    let text = [
        i!(section!(Text)),
    ];

    Asm::from_instructions(data, bss, text)
}