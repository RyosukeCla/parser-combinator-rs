pub mod parser;
use crate::parser::{
    parse, Char, Choice, ExtractMap, FlattenMap, Kind, Lazy, Many, RegExp, Seq, Token, UnwrapMap,
    WrapMap,
};

#[derive(Clone, Debug)]
enum Symbol {
    Num,
    Op,
    Expr,
}

fn expression_example() {
    let spaces = Many(&Token(" "));
    let num = Kind(
        &UnwrapMap(&ExtractMap(
            &Seq(&spaces)
                .and(&RegExp(r"([1-9][0-9]*|[0-9])"))
                .and(&spaces),
            1,
        )),
        Symbol::Num,
    );
    let operator = Kind(&Char("+-"), Symbol::Op);
    let parenthesis = Lazy();
    let atom = Choice(&num).or(&parenthesis);
    let expression =
        FlattenMap(&Seq(&WrapMap(&atom)).and(&FlattenMap(&Many(&Seq(&operator).and(&atom)))));
    let paren_open = Seq(&spaces).and(&Token("(")).and(&spaces);
    let paren_close = Seq(&spaces).and(&Token(")")).and(&spaces);

    parenthesis.set_parser(&FlattenMap(&ExtractMap(
        &Seq(&paren_open).and(&expression).and(&paren_close),
        1, // extract expression
    )));

    let parser = Kind(&expression, Symbol::Expr); // grant Expression label

    let targets = vec![
        "10+20-(3+1-(4))",
        "hoge",
        "1+2-(3+1",
        "0-3+(((3)))",
        "1 + 2 + ( 20 + 3 )",
    ];

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

pub fn main() {
    expression_example();
}
