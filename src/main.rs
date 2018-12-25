pub mod parser;
use crate::parser::{
    parse, Char, Choice, ExtractMap, FlattenMap, Lazy, Many, RegExp, Seq, Token, WrapMap,
};

pub fn main() {
    let num = RegExp(r"([0-9]|[1-9][0-9]*)");
    let operator = Char("+-");
    let parenthesis = Lazy();
    let atom = Choice(&num).or(&parenthesis);
    let expression =
        FlattenMap(&Seq(&WrapMap(&atom)).and(&FlattenMap(&Many(&Seq(&operator).and(&atom)))));

    parenthesis.set_parser(&FlattenMap(&ExtractMap(
        &Seq(&Token("(")).and(&expression).and(&Token(")")),
        1, // extract expression
    )));

    let targets = vec!["1+2-(3+1-(4))", "hoge", "1+2-(3+1", "0-3+(((3)))"];

    for target in targets {
        println!("[In]:\n   {}\n", target);
        match parse(&expression, target) {
            Ok(res) => {
                println!("[Out]:\n   {}\n", res);
            }
            Err(message) => {
                println!("[Out]:\n   {}\n", message);
            }
        }
    }
}
