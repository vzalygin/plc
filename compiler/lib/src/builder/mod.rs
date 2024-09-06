use std::{env, fs::File, io::Write, path::PathBuf, process::Command};
use anyhow::{anyhow, Result};

use crate::translator::Asm;

const TMP_SUBDIR: &str = "plc";

pub fn link_to_executable(
    object_files_paths: Vec<PathBuf>,
    output_path: PathBuf
) -> Result<PathBuf> {
    {
        let output_path = output_path.to_str()
            .ok_or(anyhow!("path contains non-utf8 characters"))?;
    
        let mut ld_command = Command::new("ld");
        let mut ld_command = ld_command
            .args(["-dynamic-linker", "/lib64/ld-linux-x86-64.so.2"])
            .args(["-o", output_path])
            .arg("-lc");

        for object_files_path in object_files_paths {
            ld_command = ld_command.arg(
                object_files_path
                    .to_str()
                    .ok_or(anyhow!("path contains non-utf8 characters"))?
            )
        }

        let ld_exit_code: std::process::ExitStatus = ld_command.status()?;

        if !ld_exit_code.success() {
            let msg = match ld_exit_code.code() {
                Some(code) => format!("ld returned code {code}"),
                None => "ld was interrupted".to_string(),
            };

            return Err(anyhow!(msg));
        }
    }

    Ok(output_path)
}

pub fn make_object_file(
    asm_file_path: PathBuf,
    output_path: PathBuf,
) -> Result<PathBuf> {
    {
        let output_path = output_path.to_str()
            .ok_or(anyhow!("path contains non-utf8 characters"))?;
        let asm_file_path = asm_file_path.to_str()
            .ok_or(anyhow!("path contains non-utf8 characters"))?;

        let nasm_exit_status = Command::new("nasm")
            .args(["-f", "elf64"])
            .args(["-o", output_path])
            .arg(asm_file_path)
            .status()?;

        if !nasm_exit_status.success() {
            let msg = match nasm_exit_status.code() {
                Some(code) => format!("nasm returned code {code}"),
                None => "nasm was interrupted".to_string(),
            };

            return Err(anyhow!(msg));
        }
    }

    Ok(output_path)
}

pub fn make_asm(
    asm: Asm,
    output: PathBuf,
) -> Result<PathBuf> {
    let code = asm.into_assembly();

    let mut file = File::create(&output)?;

    let _ = file.write(code.as_bytes())?;

    Ok(output)
}

pub fn make_tmp_asm(
    asm: Asm,
) -> Result<PathBuf> {
    let output = env::temp_dir()
        .join(TMP_SUBDIR)
        .join(uuid::Uuid::new_v4().to_string());

    make_asm(asm, output)
}
