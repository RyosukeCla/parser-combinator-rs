use crate::parser::base::{Parser, Type};
use crate::parser::map;

/**
 *  Unwrap Map
 *  [a] -> a
 */
pub fn build<T: Clone, P: Parser<T>>(parser: &P) -> map::Map<T> {
  map::build(parser, move |node| {
    let children = match node.value {
      Type::Arr(children) => children,
      _ => panic!("Coundn't unwrap: node.value is not Type:Arr"),
    };

    let extraction = &children[0];
    extraction.clone()
  })
}
