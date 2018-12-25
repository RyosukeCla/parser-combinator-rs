pub mod parser;
use crate::parser::{
    parse, Char, Choice, ExtractMap, FlattenMap, Kind, Lazy, Many, RegExp, Seq, Token, WrapMap,
};

#[derive(Clone, Debug)]
enum Symbol {
    Num,
    Op,
    Expr,
}

pub fn main() {
    let num = Kind(&RegExp(r"([0-9]|[1-9][0-9]*)"), Symbol::Num);
    let operator = Kind(&Char("+-"), Symbol::Op);
    let parenthesis = Lazy();
    let atom = Choice(&num).or(&parenthesis);
    let expression =
        FlattenMap(&Seq(&WrapMap(&atom)).and(&FlattenMap(&Many(&Seq(&operator).and(&atom)))));

    parenthesis.set_parser(&FlattenMap(&ExtractMap(
        &Seq(&Token("(")).and(&expression).and(&Token(")")),
        1, // extract expression
    )));

    let parser = Kind(&expression, Symbol::Expr); // grant Expression label

    let targets = vec!["1+2-(3+1-(4))", "hoge", "1+2-(3+1", "0-3+(((3)))"];

    for target in targets {
        println!("[In]:\n   {}\n", target);
        match parse(&parser, target) {
            Ok(res) => {
                println!("[Out]:\n   {}\n", res);
            }
            Err(message) => {
                println!("[Out]:\n   {}\n", message);
            }
        }
    }
}
