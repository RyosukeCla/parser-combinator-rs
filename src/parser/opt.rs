use crate::parser::base::{Parser, State};

pub struct Opt {
  parser: Box<Parser>,
}

pub fn build<P: Parser>(parser: &P) -> Opt {
  Opt {
    parser: parser.box_clone(),
  }
}

impl Parser for Opt {
  fn box_clone(&self) -> Box<Parser> {
    Box::new(Opt {
      parser: self.parser.box_clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State {
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
