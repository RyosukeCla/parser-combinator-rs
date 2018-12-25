use crate::parser::base::{Node, Parser, Type};
use crate::parser::map;

/**
 *  Wrap Map
 *  [a1, b1, c1, ..., a2, b2, c2, ...]
 *    to
 *  [[a1, b1, c1, ..., a2, b2, c2, ...]]
 */
pub fn build<T: Clone, P: Parser<T>>(parser: &P) -> map::Map<T> {
  map::build(
    parser,
    Box::new(|node| Node {
      value: Type::Arr(vec![node]),
      kind: None,
    }),
  )
}
