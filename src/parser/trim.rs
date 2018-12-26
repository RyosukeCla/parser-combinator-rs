use crate::parser::base::Parser;
use crate::parser::map;
use crate::parser::{extract_map, many, seq};

/**
 *  Trim "a" by "-":
 *    "--a-" -> "a"
 */
pub fn build<T: Clone + 'static, P: Parser<T>, B: Parser<T>>(parser: &P, by: &B) -> map::Map<T> {
  extract_map(&seq(&many(by)).and(parser).and(&many(by)), 1)
}
