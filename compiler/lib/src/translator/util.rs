pub struct LabelGenerator {
    count: u64,
}

impl LabelGenerator {
    pub fn default() -> LabelGenerator {
        LabelGenerator { count: 0 }
    }

    pub fn get_nameless_label(&mut self) -> String {
        self.count += 1;
        format!("$lb{}", self.count)
    }
}
