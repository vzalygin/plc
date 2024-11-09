#[allow(dead_code)]
mod util;

#[cfg(test)]
mod tests {
    use crate::util::Compiler;

    use anyhow::Result;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref compiler: Compiler = Compiler::make()
            .unwrap_or_else(|e| { panic!("Failed to build a compiler with: {}", e) });
    }

    #[test]
    fn print_command() -> Result<()> {
        let input = "1 .";
        let exp = "1\n";

        let act = &compiler.compile(input)?.and_execute_once()?;

        assert_eq!(exp, act);
        Ok(())
    }

    #[test]
    fn add_command() -> Result<()> {
        let input = "1 2 + .";
        let exp = "3\n";

        let act = &compiler.compile(input)?.and_execute_once()?;

        assert_eq!(exp, act);
        Ok(())
    }

    #[test]
    fn help_message() -> Result<()> {
        let input = ["-h"];
        let exp = "postfix language compiler\n\nUsage: plc [OPTIONS] <FILE>\n\nArguments:\n  <FILE>  \n\nOptions:\n  -S, --compile-only   Only compile file to nasm; do not assemble or link\n  -c, --assemble-only  Compile and assemble, but do not link\n  -o, --output <FILE>  Place the output file into FILE\n  -h, --help           Print help\n  -V, --version        Print version\n";

        let act = compiler.run_command(input)?;

        assert_eq!(act, exp);
        Ok(())
    }
}
