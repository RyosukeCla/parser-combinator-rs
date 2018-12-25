pub mod parser;
use crate::parser::{
    parse, Char, Choice, ExtractMap, FlattenMap, Kind, Lazy, Many, RegExp, Seq, Token, Trim,
    TypeMap, UnwrapMap, WrapMap,
};

#[derive(Clone, Debug)]
enum MyType {}

const NUM: &str = "Num";
const OP: &str = "Op";
const EXPR: &str = "Expr";

fn expression_example() {
    let spaces = Many(&Token(" "));
    let num = Kind(
        &TypeMap::<_, _, i32>(&Trim(&RegExp(r"([1-9][0-9]*|[0-9])"), &Token(" "))),
        NUM,
    );

    let operator = Kind(&Char("+-"), OP);
    let parenthesis = Lazy::<MyType>();
    let atom = Choice(&num).or(&parenthesis);
    let expression =
        FlattenMap(&Seq(&WrapMap(&atom)).and(&FlattenMap(&Many(&Seq(&operator).and(&atom)))));
    let paren_open = Seq(&spaces).and(&Token("(")).and(&spaces);
    let paren_close = Seq(&spaces).and(&Token(")")).and(&spaces);

    // FlattenMap
    parenthesis.set_parser(&UnwrapMap(&ExtractMap(
        &Seq(&paren_open).and(&expression).and(&paren_close),
        1, // extract expression
    )));

    let parser = Kind(&expression, EXPR); // grant Expression label

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
