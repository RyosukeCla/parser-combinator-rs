pub mod parser;
use crate::parser::{
    parse, Char, Choice, FlattenMap, Lazy, Many, Map, Node, RegExp, Seq, Token, WrapMap,
};

pub fn main() {
    let num = RegExp(r"([0-9]|[1-9][0-9]*)");
    let operator = Char("+-");
    let parenthesis = Lazy();
    let atom = Choice(&num).or(&parenthesis);
    let expression =
        FlattenMap(&Seq(&WrapMap(&atom)).and(&FlattenMap(&Many(&Seq(&operator).and(&atom)))));

    parenthesis.set_parser(&Map(
        &Seq(&Token("(")).and(&expression).and(&Token(")")),
        Box::new(|node| {
            // extract Expression
            let children = node.children.unwrap();
            let expression = &children[1];
            let children = expression.children.as_ref().unwrap();

            Node {
                value: None,
                children: Some(children.clone()),
            }
        }),
    ));

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
