mod base;
mod char;
mod choice;
mod extract_map;
mod flatten_map;
mod kind;
mod lazy;
mod many;
mod map;
mod opt;
mod regexp;
mod seq;
mod token;
mod trim;
mod type_map;
mod unwrap_map;
mod wrap_map;

pub use self::base::{Node, Parser, State, Type};
pub use self::char::build as Char;
pub use self::choice::build as Choice;
pub use self::extract_map::build as ExtractMap;
pub use self::flatten_map::build as FlattenMap;
pub use self::kind::build as Kind;
pub use self::lazy::build as Lazy;
pub use self::many::build as Many;
pub use self::map::build as Map;
pub use self::opt::build as Opt;
pub use self::regexp::build as RegExp;
pub use self::seq::build as Seq;
pub use self::token::build as Token;
pub use self::trim::build as Trim;
pub use self::type_map::build as TypeMap;
pub use self::unwrap_map::build as UnwrapMap;
pub use self::wrap_map::build as WrapMap;

pub fn parse<T: Clone, P: Parser<T>>(parser: &P, target: &str) -> Result<Node<T>, String> {
  let result = parser.parse(target, 0);
  if result.success {
    if result.position == target.len() {
      if let Some(node) = result.node {
        return Ok(node);
      }
    } else {
      return Err(format!("Parse Error: failed at {}", 1 + result.position));
    }
  }

  Err("Parse Error: failed at 1".to_string())
}
