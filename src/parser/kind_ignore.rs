use crate::parser::base::Parser;
use crate::parser::filter;
use crate::parser::map;

/**
 * ignore specific kind:
 * [a, ignored, ignored, c] -> [ a, c ]
 */
pub fn build<T, P>(parser: &P, ignore: &'static str) -> map::Map<T>
where
  T: Clone,
  P: Parser<T>,
{
  filter::build(parser, move |node| match &node.kind {
    Some(kind) => kind != ignore,
    _ => true,
  })
}
