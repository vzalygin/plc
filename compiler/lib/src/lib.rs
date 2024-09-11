mod builder;
mod common;
mod err;
mod parser;
mod translator;

pub use {
    builder::{
        check_tmp_dir, link_to_executable_file, make_asm_file, make_object_file, make_tmp_path,
    },
    common::{Ast, Term},
    err::CompilerError,
    parser::parse,
    translator::{make_std_lib, translate},
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
