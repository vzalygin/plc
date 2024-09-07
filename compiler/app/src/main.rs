use std::{fs::File, io::Read, path::Path};

use clap::{Args, Parser};
use anyhow::{Result, anyhow};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(flatten)]
    compilation_options: CompilationOptions,

    /// Place the output file into FILE
    #[arg(short, long, value_name = "FILE")]
    output: Option<String>,

    file: String
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct CompilationOptions {
    /// Only compile file to nasm; do not assemble or link
    #[arg(short = 'S', long)]
    compile_only: bool,

    /// Compile and assemble, but do not link
    #[arg(short = 'c', long)]
    assemble_only: bool,
}

enum OpMode {
    CompileOnly,
    AssembleOnly,
    All,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let op_mode = if cli.compilation_options.assemble_only {
        OpMode::AssembleOnly
    } else if cli.compilation_options.compile_only {
        OpMode::CompileOnly
    } else {
        OpMode::All
    };

    let current_dir = std::env::current_dir()?;
    let input_file_path = current_dir.join(cli.file);
    let output_file_path = current_dir.join(match cli.output {
        Some(output_file) => output_file,
        None => "output".to_string(),
    });

    perform(
        op_mode, 
        input_file_path.as_path(), 
        output_file_path.as_path()
    )
}

fn perform(op_mode: OpMode, input_file_path: &Path, output_file_path: &Path) -> Result<()> {
    match op_mode {
        OpMode::CompileOnly => {
            compile(input_file_path, output_file_path)?;
        },
        OpMode::AssembleOnly => {
            lib::check_tmp_dir()?;

            let asm_tmp_path = lib::make_tmp_path();

            let assemble_result = compile(input_file_path, asm_tmp_path.as_path())
                .and_then(|_| {
                    assemble(asm_tmp_path.as_path(), output_file_path)
                });

            let _ = std::fs::remove_file(asm_tmp_path);

            assemble_result?
        },
        OpMode::All => {
            lib::check_tmp_dir()?;

            let asm_tmp_path = lib::make_tmp_path();
            let object_tmp_path = lib::make_tmp_path();
            let stdlib_tmp_path = lib::make_tmp_path(); // TODO: precompile

            let compilation_result = {
                compile(
                    input_file_path,
                    asm_tmp_path.as_path()
                ).and_then(|_| {
                    assemble_stdlib(stdlib_tmp_path.as_path())
                }).and_then(|_| {
                    assemble(
                        asm_tmp_path.as_path(), 
                        object_tmp_path.as_path()
                    )
                }).and_then(|_| { 
                    link(
                        &[object_tmp_path.as_path(), stdlib_tmp_path.as_path()],
                        output_file_path
                    ) 
                })
            };

            let _ = std::fs::remove_file(asm_tmp_path);
            let _ = std::fs::remove_file(object_tmp_path);
            let _ = std::fs::remove_file(stdlib_tmp_path);

            compilation_result?
        },
    };

    Ok(())
}

fn compile(input_file_path: &Path, output_file_path: &Path) -> Result<()> {
    // TODO: move asm read functionality to lib
    let mut input = String::new();
    File::open(input_file_path)?.read_to_string(&mut input)?;

    let ast = lib::parse(input.as_str())
        .map_err(|e| anyhow!(e.to_string()))?;
    let asm = lib::translate(&ast);
    lib::make_asm_file(asm, output_file_path)?;

    Ok(())
}

fn assemble_stdlib(output_file_path: &Path) -> Result<()> {
    let stdlib = lib::make_std_lib();
    let asm_tmp_path = lib::make_tmp_path();
    
    let assemble_result = lib::make_asm_file(stdlib, asm_tmp_path.as_path())
        .and_then(|_| {
            lib::make_object_file(
                asm_tmp_path.as_path(), 
                output_file_path
            ).map(|_| {  })
        });

    let _ = std::fs::remove_file(asm_tmp_path);
    
    assemble_result
}

fn assemble(input_file_path: &Path, output_file_path: &Path) -> Result<()> {
    lib::make_object_file(
        input_file_path, 
        output_file_path
    ).map(|_| {  })
}

fn link(input_file_paths: &[&Path], output_file_path: &Path) -> Result<()> {
    lib::link_to_executable_file(
        input_file_paths, 
        output_file_path
    ).map(|_| {  })
}
