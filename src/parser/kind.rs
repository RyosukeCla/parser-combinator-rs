use crate::parser::base::{Node, Parser};
use crate::parser::map;

/**
 *  Grant Kind
 */
pub fn build<T: Clone + 'static, P: Parser<T>>(parser: &P, name: &'static str) -> map::Map<T> {
  map::build(
    parser,
    Box::new(move |node| Node {
      value: node.value,
      kind: Some(name.to_string()),
    }),
  )
}
