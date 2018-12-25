use crate::parser::base::{Parser, State};

pub struct Choice {
  parsers: Vec<Box<Parser>>,
}

pub fn build<P: Parser>(parser: &P) -> Choice {
  Choice {
    parsers: vec![parser.box_clone()],
  }
}

impl Choice {
  pub fn or<P: Parser>(mut self, parser: &P) -> Self {
    self.parsers.push(parser.box_clone());
    self
  }
}

impl Parser for Choice {
  fn box_clone(&self) -> Box<Parser> {
    let mut parsers: Vec<Box<Parser>> = vec![];

    for parser in self.parsers.iter() {
      parsers.push(parser.box_clone());
    }

    Box::new(Choice { parsers: parsers })
  }

  fn parse(&self, target: &str, position: usize) -> State {
    for parser in self.parsers.iter() {
      let parsed = parser.parse(target, position);

      if parsed.success {
        return parsed;
      }
    }

    State {
      success: false,
      node: None,
      position: position,
    }
  }
}
