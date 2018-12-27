extern crate parser_comb;
use parser_comb::parser::{map, regexp, seq, token, type_map, Node, ParserCombinator, Type};

#[derive(Clone, Debug)]
enum ExtendedType {
  Complex32(i32, i32),
}

pub fn main() {
  let num = type_map::<_, _, i32>(&regexp(r"([1-9][0-9]*|[0-9])"));
  let plus = token("+");
  let imaginary = token("i");
  let expr = seq(&num).and(&plus).and(&num).and(&imaginary);
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
}
