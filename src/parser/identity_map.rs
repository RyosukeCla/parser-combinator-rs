use crate::parser::base::{Parser, State};

pub struct IdentityMap<T: Clone> {
  parser: Box<Parser<T>>,
}

pub fn build<T, P>(parser: &P) -> IdentityMap<T>
where
  T: Clone,
  P: Parser<T>,
{
  IdentityMap {
    parser: parser.box_clone(),
  }
}

impl<T: Clone + 'static> Parser<T> for IdentityMap<T> {
  fn box_clone(&self) -> Box<Parser<T>> {
    Box::new(IdentityMap {
      parser: self.parser.box_clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State<T> {
    let parsed = self.parser.parse(target, position);

    parsed
  }
}
