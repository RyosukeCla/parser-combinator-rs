pub mod parser;
use crate::parser::{
  char, choice, extract, filter, flatten, kind, lazy, many, map, parse, regexp, seq, token, trim,
  type_map, wrap, Node, ParserCombinator, Type,
};

#[derive(Clone, Debug)]
enum ExtendedType {
  Complex32(i32, i32),
}

const NUM: &str = "Num";
const OP: &str = "Op";
const EXPR: &str = "Expr";

fn complex_number() {
  let num = type_map::<_, _, i32>(&regexp(r"([1-9][0-9]*|[0-9])"));
  let plus = token("+");
  let imaginary = token("i");
  let expr = seq(&num).and(&plus).and(&num).and(&imaginary);
  let only_num = filter(&expr, |node| match node.value {
    Type::I32(_) => true,
    _ => false,
  });
  let complex = map(&expr, |node| match node.value {
    Type::Arr(children) => {
      let re = match children[0].value {
        Type::I32(re) => re,
        _ => panic!("err"),
      };
      let im = match children[2].value {
        Type::I32(re) => re,
        _ => panic!("err"),
      };

      Node {
        value: Type::Val(ExtendedType::Complex32(re, im)),
        kind: None,
      }
    }
    _ => panic!("Error"),
  });

  let parser: ParserCombinator<ExtendedType> = ParserCombinator::new(&complex);

  let target = "100+100i";
  println!("[In]:\n{}\n", target);
  println!("[Out]:\n{:#?}\n", parser.parse(target).unwrap());
  println!("[Out]:\n{:#?}\n", parse(&only_num, target).unwrap());
}

fn expression_example() {
  let space = token(" ");
  let num = kind(
    &type_map::<_, _, i32>(&trim(&regexp(r"([1-9][0-9]*|[0-9])"), &space)),
    NUM,
  );
  let operator = kind(&char("+-*/"), OP);
  let parenthesis = lazy();
  let atom = choice(&num).or(&parenthesis);
  let expression = kind(
    &flatten(&seq(&wrap(&atom)).and(&flatten(&many(&seq(&operator).and(&atom))))),
    EXPR,
  );
  let paren_open = trim(&token("("), &space);
  let paren_close = trim(&token(")"), &space);

  parenthesis.set_parser(&extract(
    &seq(&paren_open).and(&expression).and(&paren_close),
    1, // extract expression
  ));

  let parser: ParserCombinator = ParserCombinator::new(&expression);

  let targets = vec![
    "10+20-(3+1-(4))",
    "hoge",
    "1+2-(3+1",
    "0-3+(((3)))",
    "1 + 2 + ( 20 + 3 ) / (30 - 20) * (10 - 5)",
    "1 + 2 - (3 * 4) / (5)",
  ];

  for target in targets {
    println!("[In]:\n   {}\n", target);
    match parser.parse(target) {
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
  complex_number();
  expression_example();
}
