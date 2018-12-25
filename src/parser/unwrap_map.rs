use crate::parser::map;
use crate::parser::Parser;

/**
 *  Unwrap Map
 *  [a] -> a
 */
pub fn build<K: Clone, P: Parser<K>>(parser: &P) -> map::Map<K> {
  map::build(
    parser,
    Box::new(move |node| {
      let children = node.children.unwrap();
      let extraction = &children[0];

      extraction.clone()
    }),
  )
}
