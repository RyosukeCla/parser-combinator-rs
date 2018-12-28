use crate::parser::base::Parser;
use crate::parser::map;
use crate::parser::{flatten, kind, kind_ignore, many, seq, wrap};

const DEL_FOR_SEP: &str = "DEL_FOR_SEP";

/**
 *  Sep "a,b,c,d" by "," -> [a, b, c, d]
 */
pub fn build<T: Clone + 'static, P: Parser<T>, B: Parser<T>>(parser: &P, by: &B) -> map::Map<T> {
  let del = kind(by, DEL_FOR_SEP);
  kind_ignore(
    &flatten(&seq(&wrap(parser)).and(&flatten(&many(&seq(&del).and(parser))))),
    DEL_FOR_SEP,
  )
}
