use crate::parser::map;
use crate::parser::{Node, Parser};

/**
 *  Flatten Map
 *  [
 *    [a1, b1, c1...],
 *    [a2, b2, c2...],
 *  ]
 *    to
 *  [a1, b1, c1, ..., a2, b2, c2, ...]
 */
pub fn build<K: Clone, P: Parser<K>>(parser: &P) -> map::Map<K> {
  map::build(
    parser,
    Box::new(|node| {
      let mut nodes: Vec<Node<K>> = vec![];
      let children = node.children.unwrap();

      for child in children {
        let grand_children = child.children.as_ref().unwrap();
        for grand_child in grand_children {
          nodes.push(grand_child.clone());
        }
      }

      Node {
        value: None,
        children: Some(nodes),
        kind: None,
      }
    }),
  )
}
