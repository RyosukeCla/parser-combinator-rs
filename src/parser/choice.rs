use crate::parser::base::{Parser, State};

pub struct Choice<T> {
  parsers: Vec<Box<Parser<T>>>,
}

pub fn build<T: Clone, P: Parser<T>>(parser: &P) -> Choice<T> {
  Choice {
    parsers: vec![parser.box_clone()],
  }
}

impl<T: Clone> Choice<T> {
  pub fn or<P: Parser<T>>(mut self, parser: &P) -> Self {
    self.parsers.push(parser.box_clone());
    self
  }
}

impl<T: Clone + 'static> Parser<T> for Choice<T> {
  fn box_clone(&self) -> Box<Parser<T>> {
    let mut parsers: Vec<Box<Parser<T>>> = vec![];

    for parser in self.parsers.iter() {
      parsers.push(parser.box_clone());
    }

    Box::new(Choice { parsers: parsers })
  }

  fn parse(&self, target: &str, position: usize) -> State<T> {
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
