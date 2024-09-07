mod parser;
mod translator;
mod builder;
mod common;
mod err;

pub use {
    parser::parse,
    translator::{
        translate,
        make_std_lib,
    },
    builder::{
        make_asm_file,
        make_object_file,
        link_to_executable_file,
        make_tmp_path,
        check_tmp_dir,
    },
    common::{
        Ast,
        Term,
    },
    err::CompilerError,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2+2;
        assert_eq!(result, 4);
    }
}
