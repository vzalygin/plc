use core::str;
use std::{env, ffi::OsStr, path::PathBuf, process::{Command, ExitStatus, Stdio}};
use anyhow::{anyhow, Result};

const TMP_SUBDIR: &str = "plc_e2e";

pub struct Output {
    status: ExitStatus,
    output: String,
}

#[derive(Clone, Debug)]
pub struct Compiler {
    executable: PathBuf,
}

impl Compiler {
    pub fn new(executable: PathBuf) -> Compiler {
        Compiler { executable }
    }

    pub fn make() -> Result<Compiler> {
        let build = Command::new("cargo")
            .args(["build", "--verbose", "--manifest-path", "../app/Cargo.toml"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        let pwd = Command::new("pwd").output()?;
    
        if !build.success() {
            return Err(anyhow!("Build returned non-zero exit code: {}", map_err_status(build)))
        }

        if !pwd.status.success() {
            return Err(anyhow!("Pwd command returned non-zero exit code: {}", map_err_status(pwd.status)))
        }

        check_tmp_dir()?;

        let executable = String::from_utf8(pwd.stdout)? + "/../target/debug/plc"; 
        Ok(Compiler::new(PathBuf::from(executable)))
    }

    pub fn run_command<A, S>(&self, args: A) -> Result<String>
    where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr> {
        run_command(
            &mut Command::new(self.executable.as_path())
                .args(args)
        )
    }

    pub fn compile(&self, input: &str) -> Result<String> {
        self.translate(input, [""], true)
    }

    pub fn translate<A, S>(&self, input: &str, args: A, execute: bool) -> Result<String>
    where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr> {
        let input_path = make_tmp_path();
        let output_path = make_tmp_path();

        std::fs::write(&input_path, input)?;
        
        let run_result = run_command(
            &mut Command::new(self.executable.as_path())
                .args(args)
                .args(["--output"]).args([&output_path, &input_path])
        );

        let output = if execute {

        } else {
            std::fs::read_to_string(&output_path)?;
        };

        let _ = std::fs::remove_file(&input_path);
        let _ = std::fs::remove_file(&output_path);

        run_result?;

        Ok(output)
    }
}

fn map_err_status(status: ExitStatus) -> String {
    status.code()
        .map(|x| x.to_string())
        .unwrap_or("no-code".to_string())
}

fn make_tmp_path() -> PathBuf {
    env::temp_dir()
        .join(TMP_SUBDIR)
        .join(uuid::Uuid::new_v4().to_string())
}

fn check_tmp_dir() -> Result<()> {
    std::fs::create_dir_all(env::temp_dir().join(TMP_SUBDIR))
        .map_err(|e| e.into())
}

fn run_command(prepared_command: &mut Command) -> Result<String> {
    let output = prepared_command
        .arg("2>&1")
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(anyhow!(map_err_status(output.status)))
    }
}
