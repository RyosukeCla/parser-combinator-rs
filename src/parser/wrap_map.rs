use crate::parser::map;
use crate::parser::{Node, Parser};

/**
 *  Wrap Map
 *  [a1, b1, c1, ..., a2, b2, c2, ...]
 *    to
 *  [[a1, b1, c1, ..., a2, b2, c2, ...]]
 */
pub fn build<P: Parser>(parser: &P) -> map::Map {
  map::build(
    parser,
    Box::new(|node| Node {
      value: None,
      children: Some(vec![node.clone()]),
    }),
  )
}
