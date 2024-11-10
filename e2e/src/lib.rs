#[allow(dead_code)]
mod util;

#[cfg(test)]
mod tests {
    use std::process::Command;

    use crate::util::{osstr_to_str, run_command, Compiler};

    use anyhow::Result;
    use elf::{endian::AnyEndian, ElfBytes};
    use is_executable::IsExecutable;
    use lazy_static::lazy_static;
    use parameterized::parameterized;

    lazy_static! {
        static ref compiler: Compiler = Compiler::make()
            .unwrap_or_else(|e| { panic!("Failed to make a compiler with: {}", e) });
    }

    #[test]
    fn print_operator() -> Result<()> {
        compile_run_assert("1 .", "1\n")
    }

    #[test]
    fn ignore_comments() -> Result<()> {
        compile_run_assert("1 2 # 5 * \n+ .", "3\n")
    }

    #[test]
    fn add_operator() -> Result<()> {
        compile_run_assert("1 2 + .", "3\n")
    }

    #[test]
    fn sub_operator() -> Result<()> {
        compile_run_assert("1 2 - .", "-1\n")
    }

    #[test]
    fn mul_operator() -> Result<()> {
        compile_run_assert("3 2 * .", "6\n")
    }

    #[test]
    fn div_operator() -> Result<()> {
        compile_run_assert("6 2 / .", "3\n")
    }

    #[test]
    fn div_operator_integer_result() -> Result<()> {
        compile_run_assert("5 2 / .", "2\n")
    }

    #[test]
    fn dup_operator() -> Result<()> {
        compile_run_assert("2 dup . .", "2\n2\n")
    }

    #[test]
    fn drop_operator() -> Result<()> {
        compile_run_assert("1 2 drop .", "1\n")
    }

    #[test]
    fn take_operator() -> Result<()> {
        compile_run_assert("1 2 3 2 take . . .", "1\n3\n2\n")
    }

    #[test]
    fn take_operator_no_effect() -> Result<()> {
        compile_run_assert("1 2 3 0 take . . .", "3\n2\n1\n")
    }

    #[test]
    fn make_list() -> Result<()> {
        compile_run_assert("[ 1 2 3 + + .]", "")
    }

    #[test]
    fn make_list_and_apply() -> Result<()> {
        compile_run_assert("[ 1 2 + ] ! .", "3\n")
    }

    #[test]
    fn nested_lists() -> Result<()> {
        compile_run_assert("[ [ 1 ] ] ! ! .", "1\n")
    }

    #[test]
    #[ignore]
    fn not_operator() -> Result<()> {
        compile_run_assert_many(&[("1 not .", "0\n", "1 -> 0"), ("0 not .", "1\n", "0 -> 1")])
    }

    #[test]
    fn and_operator() -> Result<()> {
        compile_run_assert_many(&[
            ("0 0 and .", "0\n", "0 and 0"),
            ("0 1 and .", "0\n", "0 and 1"),
            ("1 0 and .", "0\n", "1 and 0"),
            ("1 1 and .", "1\n", "1 and 1"),
        ])
    }

    #[test]
    fn or_operator() -> Result<()> {
        compile_run_assert_many(&[
            ("0 0 or .", "0\n", "0 or 0"),
            ("0 1 or .", "1\n", "0 or 1"),
            ("1 0 or .", "1\n", "1 or 0"),
            ("1 1 or .", "1\n", "1 or 1"),
        ])
    }

    #[test]
    fn and_operator_integer() -> Result<()> {
        compile_run_assert("5 6 and .", "4\n")
    }

    #[test]
    fn or_operator_integer() -> Result<()> {
        compile_run_assert("1 2 or .", "3\n")
    }

    #[test]
    fn to_binary_operator() -> Result<()> {
        compile_run_assert_many(&[
            ("10b .", "1\n", "10b == 1"),
            ("-10b .", "1\n", "-10b == 1"),
            ("0b .", "0\n", "0b == 0"),
        ])
    }

    #[test]
    fn equals_operator() -> Result<()> {
        compile_run_assert_many(&[
            ("10 10 == .", "1\n", "10 == 10"),
            ("10 9  == .", "0\n", "10 == 9"),
        ])
    }

    #[test]
    fn not_equals_operator() -> Result<()> {
        compile_run_assert_many(&[
            ("10 10 != .", "0\n", "10 != 10"),
            ("10 9  != .", "1\n", "10 != 9"),
        ])
    }

    #[test]
    fn greater_operator() -> Result<()> {
        compile_run_assert_many(&[
            ("11 10 > .", "1\n", "11 > 10"),
            ("10 10 > .", "0\n", "10 > 10"),
            ("9 10  > .", "0\n", "9 > 10"),
        ])
    }

    #[test]
    fn greater_equal_operator() -> Result<()> {
        compile_run_assert_many(&[
            ("11 10 >= .", "1\n", "11 >= 10"),
            ("10 10 >= .", "1\n", "10 >= 10"),
            ("9 10  >= .", "0\n", "9 >= 10"),
        ])
    }

    #[test]
    fn less_operator() -> Result<()> {
        compile_run_assert_many(&[
            ("11 10 < .", "0\n", "11 < 10"),
            ("10 10 < .", "0\n", "10 < 10"),
            ("9 10  < .", "1\n", "9 < 10"),
        ])
    }

    #[test]
    fn less_equal_operator() -> Result<()> {
        compile_run_assert_many(&[
            ("11 10 <= .", "0\n", "11 <= 10"),
            ("10 10 <= .", "1\n", "10 <= 10"),
            ("9 10  <= .", "1\n", "9 <= 10"),
        ])
    }

    #[test]
    fn make_named_list() -> Result<()> {
        compile_run_assert("[ 42 . ] :foo", "")
    }

    #[test]
    fn make_named_list_and_call() -> Result<()> {
        compile_run_assert("[ 42 . ] :foo foo!", "42\n")
    }

    #[test]
    fn make_named_num() -> Result<()> {
        compile_run_assert("42 :foo foo .", "42\n")
    }

    #[parameterized(
        flag = { "-h", "--help" }
    )]
    fn help_message(flag: &str) -> Result<()> {
        run_assert(
            &[flag],
            "postfix language compiler\n\nUsage: plc [OPTIONS] <FILE>\n\nArguments:\n  <FILE>  \n\nOptions:\n  -S, --compile-only   Only compile file to nasm; do not assemble or link\n  -c, --assemble-only  Compile and assemble, but do not link\n  -o, --output <FILE>  Place the output file into FILE\n  -h, --help           Print help\n  -V, --version        Print version\n",
        )
    }

    #[parameterized(
        flag = { "-o", "--output" }
    )]
    fn output_flag(flag: &str) -> Result<()> {
        let input_path = compiler.make_tmp_path();
        let output_path = compiler.make_tmp_path();

        std::fs::write(&input_path, "")?;

        compiler.run_command([
            flag,
            osstr_to_str(output_path.as_os_str())?,
            osstr_to_str(input_path.as_os_str())?,
        ])?;

        assert!(output_path.exists(), "output file exists");
        assert!(output_path.is_executable(), "output file is executable");

        std::fs::remove_file(&input_path)?;
        std::fs::remove_file(&output_path)?;

        Ok(())
    }

    #[parameterized(
        flag = { "-S", "--compile-only" }
    )]
    fn compile_only_flag(flag: &str) -> Result<()> {
        let input_path = compiler.make_tmp_path();
        let output_path_compiler = compiler.make_tmp_path();
        let output_path_nasm = compiler.make_tmp_path();

        std::fs::write(&input_path, "")?;

        compiler.run_command([
            flag,
            "--output",
            osstr_to_str(output_path_compiler.as_os_str())?,
            osstr_to_str(input_path.as_os_str())?,
        ])?;

        assert!(output_path_compiler.exists(), "output file exists");

        let _ = run_command(Command::new("nasm").args([
            "-f",
            "elf64",
            "-o",
            osstr_to_str(output_path_nasm.as_os_str())?,
            osstr_to_str(output_path_compiler.as_os_str())?,
        ]))?;

        std::fs::remove_file(&input_path)?;
        std::fs::remove_file(&output_path_compiler)?;
        std::fs::remove_file(&output_path_nasm)?;

        Ok(())
    }

    #[parameterized(
        flag = { "-c", "--assemble-only" }
    )]
    fn assemble_only_flag(flag: &str) -> Result<()> {
        let input_path = compiler.make_tmp_path();
        let output_path = compiler.make_tmp_path();

        std::fs::write(&input_path, "")?;

        compiler.run_command([
            flag,
            "--output",
            osstr_to_str(output_path.as_os_str())?,
            osstr_to_str(input_path.as_os_str())?,
        ])?;

        assert!(output_path.exists(), "output file exists");

        ElfBytes::<AnyEndian>::minimal_parse(std::fs::read(&output_path)?.as_slice())?;

        std::fs::remove_file(&input_path)?;
        std::fs::remove_file(&output_path)?;

        Ok(())
    }

    fn compile_run_assert(input: &str, expected_output: &str) -> Result<()> {
        let actual_output = &compiler.compile(input)?.and_execute_once()?;
        assert_eq!(expected_output, actual_output);
        Ok(())
    }

    fn compile_run_assert_with_dect(
        input: &str,
        expected_output: &str,
        description: &str,
    ) -> Result<()> {
        let actual_output = &compiler.compile(input)?.and_execute_once()?;
        assert_eq!(expected_output, actual_output, "{}", description);
        Ok(())
    }

    fn compile_run_assert_many(data: &[(&str, &str, &str)]) -> Result<()> {
        for (input, expected_output, description) in data {
            compile_run_assert_with_dect(input, expected_output, description)?
        }
        Ok(())
    }

    fn run_assert(args: &[&str], expected_output: &str) -> Result<()> {
        let actual_output = compiler.run_command(args)?;
        assert_eq!(actual_output, expected_output);
        Ok(())
    }
}
