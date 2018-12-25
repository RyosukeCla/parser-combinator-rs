use crate::parser::map;
use crate::parser::{Node, Parser};

/**
 *  Wrap Map
 *  [a1, b1, c1, ..., a2, b2, c2, ...]
 *    to
 *  [[a1, b1, c1, ..., a2, b2, c2, ...]]
 */
pub fn build<K: Clone, P: Parser<K>>(parser: &P) -> map::Map<K> {
  map::build(
    parser,
    Box::new(|node| Node {
      value: None,
      children: Some(vec![node.clone()]),
      kind: None,
    }),
  )
}
