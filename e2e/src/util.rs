use anyhow::{anyhow, Error, Result};
use core::str;
use std::{
    env,
    ffi::OsStr,
    io::Write,
    path::PathBuf,
    process::{Command, ExitStatus, Output, Stdio},
};

#[derive(Debug)]
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

    pub fn and_execute_once(self, stdin: &str) -> Result<String> {
        if let Some(file) = &self.output_file {
            let result = run_command(&mut Command::new(file), stdin);
            let _ = std::fs::remove_file(file);
            match result {
                Ok(result) => Ok(result),
                Err(e) => Err(anyhow!("Execution error: {}", e)),
            }
        } else {
            Err(anyhow!("no output file"))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Compiler {
    executable: PathBuf,
    tmp_dir: PathBuf,
}

impl Compiler {
    pub fn new(executable: PathBuf, tmp_dir: PathBuf) -> Compiler {
        Compiler {
            executable,
            tmp_dir,
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
                "Compiler build failure: {}",
                map_err_output(&build)
            ));
        }

        let mut executable = env::current_dir()?;
        executable.push("../target/debug/plc");
        let compiler = Compiler::new(executable, env::temp_dir());

        compiler.check_tmp_dir()?;

        Ok(compiler)
    }

    pub fn run_command<A, S>(&self, args: A, stdin: &str) -> Result<String>
    where
        A: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        run_command(Command::new(self.executable.as_path()).args(args), stdin)
    }

    pub fn compile(&self, input: &str) -> Result<CompilationResult> {
        self.compile_with_args(input, &[])
    }

    pub fn compile_with_args(&self, input: &str, args: &[&str]) -> Result<CompilationResult> {
        let input_path = self.make_tmp_path();
        let output_path = self.make_tmp_path();

        std::fs::write(input_path.clone(), input)?;

        let compilation_output = Command::new(self.executable.as_path())
            .args(args)
            .args(["--output"])
            .args([&output_path, &input_path])
            .output();

        let _ = std::fs::remove_file(&input_path);

        let compilation_output = compilation_output?;
        if !compilation_output.status.success() {
            return Err(anyhow!(
                "Compilation failure: {}",
                map_err_output(&compilation_output)
            ));
        }

        let output_file_path = if output_path.exists() {
            Some(output_path)
        } else {
            None
        };

        Ok(CompilationResult::new(compilation_output, output_file_path))
    }

    pub fn make_tmp_path(&self) -> PathBuf {
        env::temp_dir()
            .join(&self.tmp_dir)
            .join(uuid::Uuid::new_v4().to_string())
    }

    fn check_tmp_dir(&self) -> Result<()> {
        std::fs::create_dir_all(env::temp_dir().join(&self.tmp_dir)).map_err(|e| e.into())
    }
}

pub fn run_command(prepared_command: &mut Command, stdin: &str) -> Result<String> {
    let child = prepared_command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child.stdin.as_ref().unwrap().write_all(stdin.as_bytes())?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(map_err_output(&output))
    }
}

fn map_err_output(output: &Output) -> Error {
    anyhow!(
        "Returned non-zero exit code: {}\nstdout: {:?}\nstderr: {:?}",
        map_err_status(output.status),
        str::from_utf8(output.stdout.as_slice()),
        str::from_utf8(output.stderr.as_slice())
    )
}

fn map_err_status(status: ExitStatus) -> String {
    status
        .code()
        .map(|x| x.to_string())
        .unwrap_or("no-code".to_string())
}

pub fn osstr_to_str(osstr: &OsStr) -> Result<&str> {
    osstr
        .to_str()
        .ok_or("invalid output path")
        .map_err(|e| anyhow!("{}", e))
}
