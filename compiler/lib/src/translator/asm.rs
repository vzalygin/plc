use x64asm::Instruction;

pub struct Asm {
    pub data: Vec<Instruction>,
    pub bss: Vec<Instruction>,
    pub text: Vec<Instruction>,
}

impl Asm {
    pub fn empty() -> Asm {
        Asm { data: vec![], bss: vec![], text: vec![] }
    }

    pub fn from_instructions(
        data: Vec<Instruction>,
        bss: Vec<Instruction>,
        text: Vec<Instruction>,
    ) -> Asm {
        Asm { data, bss, text }
    }

    pub fn append(self, asm: Asm) -> Asm {
        let mut data = self.data;
        let mut bss = self.bss;
        let mut text = self.text;

        data.extend(asm.data);
        bss.extend(asm.bss);
        text.extend(asm.text);

        Self::from_instructions(data, bss, text)
    }

    pub fn from_text(text: Vec<Instruction>) -> Asm {
        Self::from_instructions(vec![], vec![], text)
    }
}
