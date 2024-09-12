use x64asm::{
    convert::{Separator, ToAssembly},
    Instruction,
};

pub struct Asm {
    pub rodata: Vec<Instruction>,
    pub bss: Vec<Instruction>,
    pub text: Vec<Instruction>,
    pub text_tail: Vec<Instruction>,
}

impl Asm {
    pub fn empty() -> Asm {
        Asm {
            rodata: vec![],
            bss: vec![],
            text: vec![],
            text_tail: vec![],
        }
    }

    pub fn new_with_tail(
        rodata: Vec<Instruction>,
        bss: Vec<Instruction>,
        text: Vec<Instruction>,
        text_tail: Vec<Instruction>,
    ) -> Asm {
        Asm {
            rodata,
            bss,
            text,
            text_tail,
        }
    }

    pub fn new(rodata: Vec<Instruction>, bss: Vec<Instruction>, text: Vec<Instruction>) -> Asm {
        Asm {
            rodata,
            bss,
            text,
            text_tail: vec![],
        }
    }

    pub fn append(self, asm: Asm) -> Asm {
        let mut data = self.rodata;
        let mut bss = self.bss;
        let mut text = self.text;
        let mut text_tail = self.text_tail;

        data.extend(asm.rodata);
        bss.extend(asm.bss);
        text.extend(asm.text);
        text_tail.extend(asm.text_tail);

        Self::new_with_tail(data, bss, text, text_tail)
    }

    pub fn rodata<const L: usize>(self, rodata: [Instruction; L]) -> Asm {
        let mut old_rodata = self.rodata;
        old_rodata.extend(rodata);
        Self::new_with_tail(old_rodata, self.bss, self.text, self.text_tail)
    }

    pub fn bss<const L: usize>(self, bss: [Instruction; L]) -> Asm {
        let mut old_bss = self.bss;
        old_bss.extend(bss);
        Self::new_with_tail(self.rodata, old_bss, self.text, self.text_tail)
    }

    pub fn text<const L: usize>(self, text: [Instruction; L]) -> Asm {
        let mut old_text = self.text;
        old_text.extend(text);
        Self::new_with_tail(self.rodata, self.bss, old_text, self.text_tail)
    }

    pub fn text_tail<const L: usize>(self, text_tail: [Instruction; L]) -> Asm {
        let mut old_text_tail = self.text_tail;
        old_text_tail.extend(text_tail);
        Self::new_with_tail(self.rodata, self.bss, self.text, old_text_tail)
    }

    pub fn into_assembly(self) -> String {
        self.rodata
            .into_iter()
            .chain(self.bss)
            .chain(self.text)
            .collect::<Vec<Instruction>>()
            .to_assembly(Separator::Space)
    }
}
