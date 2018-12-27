use crate::parser::base::{Node, Parser, Type};
use crate::parser::map;

/**
 *  Except for c
 *  [ a, b, c, ... ]
 *    ->
 *  [ a, b, ... ]
 */
pub fn build<T: Clone, P, F>(parser: &P, except: usize) -> map::Map<T>
where
  P: Parser<T>,
  F: Fn(&Node<T>) -> bool + 'static,
{
  map::build(parser, move |node| {
    let children = match node.value {
      Type::Arr(mut children) => {
        children.swap_remove(except);
        children
      }
      _ => panic!("Could not extract: node.value is not Type::Arr"),
    };

    Node {
      value: Type::Arr(children),
      kind: None,
    }
  })
}
