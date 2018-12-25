use crate::parser::base::{Parser, State};

pub struct Choice<K> {
  parsers: Vec<Box<Parser<K>>>,
}

pub fn build<K: Clone, P: Parser<K>>(parser: &P) -> Choice<K> {
  Choice {
    parsers: vec![parser.box_clone()],
  }
}

impl<K: Clone> Choice<K> {
  pub fn or<P: Parser<K>>(mut self, parser: &P) -> Self {
    self.parsers.push(parser.box_clone());
    self
  }
}

impl<K: Clone + 'static> Parser<K> for Choice<K> {
  fn box_clone(&self) -> Box<Parser<K>> {
    let mut parsers: Vec<Box<Parser<K>>> = vec![];

    for parser in self.parsers.iter() {
      parsers.push(parser.box_clone());
    }

    Box::new(Choice { parsers: parsers })
  }

  fn parse(&self, target: &str, position: usize) -> State<K> {
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
