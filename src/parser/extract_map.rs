use crate::parser::base::{Node, Parser, Type};
use crate::parser::map;

/**
 *  Extract Map
 *  [ a, b, c, ... ]
 *    to
 *  [ b ]
 */
pub fn build<T: Clone, P: Parser<T>>(parser: &P, extract: usize) -> map::Map<T> {
  map::build(
    parser,
    Box::new(move |node| {
      let children = match node.value {
        Type::Arr(children) => children,
        _ => panic!("Could not extract: node.value is not Type::Arr"),
      };
      let extraction = &children[extract];

      Node {
        value: Type::Arr(vec![extraction.clone()]),
        kind: None,
      }
    }),
  )
}
