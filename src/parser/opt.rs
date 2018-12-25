use crate::parser::base::{Parser, State};

pub struct Opt<T> {
  parser: Box<Parser<T>>,
}

pub fn build<T: Clone, P: Parser<T>>(parser: &P) -> Opt<T> {
  Opt {
    parser: parser.box_clone(),
  }
}

impl<T: Clone + 'static> Parser<T> for Opt<T> {
  fn box_clone(&self) -> Box<Parser<T>> {
    Box::new(Opt {
      parser: self.parser.box_clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State<T> {
    let parsed = self.parser.parse(target, position);
    if parsed.success {
      parsed
    } else {
      State {
        success: true,
        node: None,
        position: position,
      }
    }
  }
}
