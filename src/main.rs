use compiling::{compile, CompileInfo};
use parsing::{Ast, Statement::*};

mod compiling;
mod parsing;

fn main() {
    let ast = Ast {
        program: vec![PushValue(1), PushValue(2), Add, PopOut],
    };

    // let info = CompileInfo { ast };

    // let res = compile(info).unwrap();

    // std::fs::write("./output", res.as_slice()).unwrap();
}
