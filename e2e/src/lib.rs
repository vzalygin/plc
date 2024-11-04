#[allow(dead_code)]

mod util;

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::util::Compiler;

    #[test]
    fn successful_build() -> Result<()> {
        Compiler::make().map(|_| { () })
    }

    #[test]
    fn add_command() -> Result<()> {
        let compiler = Compiler::make()?;

        compiler.compile("123");

        Ok(())
    }
}
