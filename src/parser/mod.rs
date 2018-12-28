pub mod base;
pub mod char;
pub mod choice;
pub mod except;
pub mod extract;
pub mod filter;
pub mod flatten;
pub mod identity_map;
pub mod kind;
pub mod lazy;
pub mod many;
pub mod many1;
pub mod map;
pub mod opt;
pub mod regexp;
pub mod seq;
pub mod token;
pub mod trim;
pub mod type_map;
pub mod unwrap;
pub mod wrap;

pub use self::base::{DefaultType, Node, Parser, State, Type};
pub use self::char::build as char;
pub use self::choice::build as choice;
pub use self::except::build as except;
pub use self::extract::build as extract;
pub use self::filter::build as filter;
pub use self::flatten::build as flatten;
pub use self::identity_map::build as identity_map;
pub use self::kind::build as kind;
pub use self::lazy::build as lazy;
pub use self::many::build as many;
pub use self::many1::build as many1;
pub use self::map::build as map;
pub use self::opt::build as opt;
pub use self::regexp::build as regexp;
pub use self::seq::build as seq;
pub use self::token::build as token;
pub use self::trim::build as trim;
pub use self::type_map::build as type_map;
pub use self::unwrap::build as unwrap;
pub use self::wrap::build as wrap;

pub struct ParserCombinator<T: Clone = DefaultType> {
  pub parser: Box<Parser<T>>,
}

impl<T: Clone + Sized + 'static> ParserCombinator<T> {
  pub fn new<P: Parser<T>>(parser: &P) -> Self {
    ParserCombinator {
      parser: parser.box_clone(),
    }
  }

  pub fn parse(&self, target: &str) -> Result<Node<T>, String> {
    let result = self.parser.parse(target, 0);
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
}

impl<T: Clone + 'static> Parser<T> for ParserCombinator<T> {
  fn box_clone(&self) -> Box<Parser<T>> {
    Box::new(ParserCombinator {
      parser: self.parser.box_clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State<T> {
    self.parser.as_ref().parse(target, position)
  }
}

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

pub fn debug_parse<T: Clone, P: Parser<T>>(parser: &P, target: &str, position: usize) -> State<T> {
  parser.parse(target, position)
}
