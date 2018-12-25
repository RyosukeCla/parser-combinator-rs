use crate::parser::base::{Parser, State};

pub struct Opt<K> {
  parser: Box<Parser<K>>,
}

pub fn build<K: Clone, P: Parser<K>>(parser: &P) -> Opt<K> {
  Opt {
    parser: parser.box_clone(),
  }
}

impl<K: Clone + 'static> Parser<K> for Opt<K> {
  fn box_clone(&self) -> Box<Parser<K>> {
    Box::new(Opt {
      parser: self.parser.box_clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State<K> {
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
