use crate::parser::base::{Parser, Type};
use crate::parser::filter;
use crate::parser::map;

/**
 * ignore specific kind:
 * [a, ignored, ignored, c] -> [ a, c ]
 */
pub fn build<T, P>(parser: &P) -> map::Map<T>
where
  T: Clone,
  P: Parser<T>,
{
  filter::build(parser, move |node| match &node.value {
    Type::Arr(children) => children.len() != 0,
    _ => true,
  })
}
