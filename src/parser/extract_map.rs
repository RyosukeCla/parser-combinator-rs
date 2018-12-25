use crate::parser::map;
use crate::parser::{Node, Parser};

/**
 *  Extract Map
 *  [ a, b, c, ... ]
 *    to
 *  [ b ]
 */
pub fn build<P: Parser>(parser: &P, extract: usize) -> map::Map {
  map::build(
    parser,
    Box::new(move |node| {
      let children = node.children.unwrap();
      let extraction = &children[extract];

      Node {
        value: None,
        children: Some(vec![extraction.clone()]),
      }
    }),
  )
}
