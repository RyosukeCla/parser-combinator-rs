use crate::parser::base::{Node, Parser, Type};
use crate::parser::map;

/**
 *  Flatten Map
 *  [
 *    [a1, b1, c1...],
 *    [a2, b2, c2...],
 *  ]
 *    to
 *  [a1, b1, c1, ..., a2, b2, c2, ...]
 */
pub fn build<T: Clone, P: Parser<T>>(parser: &P) -> map::Map<T> {
  map::build(
    parser,
    Box::new(|node| {
      let mut nodes: Vec<Node<T>> = vec![];
      let children = match node.value {
        Type::Arr(children) => children,
        _ => panic!("Could not flatten: node.value is not Type::Arr"),
      };

      for child in children {
        let grand_children = match child.value {
          Type::Arr(grand_children) => grand_children,
          _ => panic!("Could not flatten: node.value.value is not Type::Arr"),
        };

        for grand_child in grand_children {
          nodes.push(grand_child);
        }
      }

      Node {
        value: Type::Arr(nodes),
        kind: None,
      }
    }),
  )
}
