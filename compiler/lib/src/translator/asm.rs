use x64asm::{
    convert::{Separator, ToAssembly},
    Instruction,
};

pub struct Asm {
    pub data: Vec<Instruction>,
    pub bss: Vec<Instruction>,
    pub text: Vec<Instruction>,
}

impl Asm {
    pub fn empty() -> Asm {
        Asm {
            data: vec![],
            bss: vec![],
            text: vec![],
        }
    }

    pub fn new(data: Vec<Instruction>, bss: Vec<Instruction>, text: Vec<Instruction>) -> Asm {
        Asm { data, bss, text }
    }

    pub fn from_instructions<const T1: usize, const T2: usize, const T3: usize>(
        data: [Instruction; T1],
        bss: [Instruction; T2],
        text: [Instruction; T3],
    ) -> Asm {
        Self::new(data.to_vec(), bss.to_vec(), text.to_vec())
    }

    pub fn append(self, asm: Asm) -> Asm {
        let mut data = self.data;
        let mut bss = self.bss;
        let mut text = self.text;

        data.extend(asm.data);
        bss.extend(asm.bss);
        text.extend(asm.text);

        Self::new(data, bss, text)
    }

    pub fn from_text<const L: usize>(text: [Instruction; L]) -> Asm {
        Self::new(vec![], vec![], text.to_vec())
    }

    pub fn into_assembly(self) -> String {
        self.data
            .into_iter()
            .chain(self.bss)
            .chain(self.text)
            .collect::<Vec<Instruction>>()
            .to_assembly(Separator::Space)
    }
}
