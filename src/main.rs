pub mod parser;
use crate::parser::{Char, Choice, Lazy, Many, Map, Node, Parser, RegExp, Seq, Token};

pub fn main() {
    let num = RegExp::new(r"([1-9][0-9]*)");
    let operator = Char::new("+-");
    let parenthesis = Lazy::new();
    let atom = Choice::new(&num).or(&parenthesis);
    let expression = Map::new(
        &Seq::new(&atom).and(&Many::new(&Seq::new(&operator).and(&atom))),
        Box::new(|node| {
            let mut nodes: Vec<Node> = vec![];
            let children = node.children.unwrap();

            // Seq(atom)
            let first_atom = &children[0];
            nodes.push(first_atom.clone());

            // .and(Many)
            let seconds = &children[1];
            let seconds = seconds.children.as_ref().unwrap();
            for second in seconds {
                // Seq
                let children = second.children.as_ref().unwrap();
                for child in children {
                    nodes.push(child.clone());
                }
            }

            Node {
                value: None,
                children: Some(nodes),
            }
        }),
    );

    parenthesis.set_parser(&Map::new(
        &Seq::new(&Token::new("("))
            .and(&expression)
            .and(&Token::new(")")),
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

    let target = "1+2-(3+1-(4))";
    println!("[In]\n{}\n", target);
    println!("[Out]\n{:#?}", expression.parse(target, 0));
}
