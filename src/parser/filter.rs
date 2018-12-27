use crate::parser::base::{Node, Parser, Type};
use crate::parser::map;

/**
 *  Filter
 *  [ a, b, c, ... ]
 *    ->
 *  [ b, c ]
 */
pub fn build<T: Clone, P, F>(parser: &P, filter: F) -> map::Map<T>
where
  P: Parser<T>,
  F: Fn(&Node<T>) -> bool + 'static,
{
  map::build(parser, move |node| {
    let children = match node.value {
      Type::Arr(children) => children.into_iter().filter(|node| filter(node)).collect(),
      _ => panic!("Could not extract: node.value is not Type::Arr"),
    };

    Node {
      value: Type::Arr(children),
      kind: None,
    }
  })
}
