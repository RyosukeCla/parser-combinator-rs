use crate::parser::base::Parser;
use crate::parser::map;
use crate::parser::{flatten, many, seq, wrap};

/**
 *  Many1
 */
pub fn build<T: Clone + 'static, P: Parser<T>>(parser: &P) -> map::Map<T> {
  flatten(&seq(&wrap(parser)).and(&many(parser)))
}
