use anyhow::{anyhow, Result};
use core::str;
use std::{
    env,
    ffi::OsStr,
    path::PathBuf,
    process::{Command, ExitStatus, Output, Stdio},
};

pub struct CompilationResult {
    pub compilation_output: Output,
    pub output_file: Option<PathBuf>,
}

impl CompilationResult {
    pub fn new(compilation_output: Output, output_file: Option<PathBuf>) -> CompilationResult {
        CompilationResult {
            compilation_output,
            output_file,
        }
    }

    pub fn and_execute_once(self) -> Result<String> {
        if let Some(file) = &self.output_file {
            let result = run_command(&mut Command::new(file));
            let _ = std::fs::remove_file(file);
            result
        } else {
            Err(anyhow!("no output file"))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Compiler {
    executable: PathBuf,
    tmp_path: PathBuf,
}

impl Compiler {
    pub fn new(executable: PathBuf, tmp_path: PathBuf) -> Compiler {
        Compiler {
            executable,
            tmp_path,
        }
    }

    pub fn make() -> Result<Compiler> {
        let build = Command::new("cargo")
            .args(["build", "--verbose", "--manifest-path", "../app/Cargo.toml"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()?;

        if !build.status.success() {
            return Err(anyhow!(
                "Build returned non-zero exit code: {}\nstdout: {:?}\nstderr: {:?}",
                map_err_status(build.status),
                build.stdout,
                build.stderr
            ));
        }

        let mut pwd = env::current_dir()?;
        let mut tmp = pwd.clone();
        pwd.push("../target/debug/plc");
        tmp.push("tmp");
        let compiler = Compiler::new(pwd, tmp);

        compiler.check_tmp_dir()?;

        Ok(compiler)
    }

    pub fn run_command<A, S>(&self, args: A) -> Result<String>
    where
        A: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        run_command(Command::new(self.executable.as_path()).args(args))
    }

    pub fn compile(&self, input: &str) -> Result<CompilationResult> {
        self.compile_with_args(input, &[])
    }

    pub fn compile_with_args(&self, input: &str, args: &[&str]) -> Result<CompilationResult> {
        let input_path = self.make_tmp_path();
        let output_path = self.make_tmp_path();

        std::fs::write(&input_path, input)?;

        let compilation_output = Command::new(self.executable.as_path())
            .args(args)
            .args(["--output"])
            .args([&output_path, &input_path])
            .output();

        let _ = std::fs::remove_file(&input_path);

        let compilation_output = compilation_output?;
        if !compilation_output.status.success() {
            return Err(anyhow!(
                "{}",
                str::from_utf8(compilation_output.stderr.as_slice())?
            ));
        }

        let output_file_path = if output_path.exists() {
            Some(output_path)
        } else {
            None
        };

        Ok(CompilationResult::new(compilation_output, output_file_path))
    }

    fn make_tmp_path(&self) -> PathBuf {
        env::temp_dir()
            .join(&self.tmp_path)
            .join(uuid::Uuid::new_v4().to_string())
    }

    fn check_tmp_dir(&self) -> Result<()> {
        std::fs::create_dir_all(env::temp_dir().join(&self.tmp_path)).map_err(|e| e.into())
    }
}

fn map_err_status(status: ExitStatus) -> String {
    status
        .code()
        .map(|x| x.to_string())
        .unwrap_or("no-code".to_string())
}

fn run_command(prepared_command: &mut Command) -> Result<String> {
    let output = prepared_command.arg("2>&1").output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(anyhow!(map_err_status(output.status)))
    }
}
