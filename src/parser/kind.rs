use crate::parser::map;
use crate::parser::{Node, Parser};

/**
 *  Grant Kind
 */
pub fn build<K: Clone + 'static, P: Parser<K>>(parser: &P, name: K) -> map::Map<K> {
  map::build(
    parser,
    Box::new(move |node| Node {
      value: node.value,
      children: node.children,
      kind: Some(name.clone()),
    }),
  )
}
