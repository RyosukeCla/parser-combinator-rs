use crate::parser::base::{Node, Parser, State, Type};

pub struct Seq<T> {
  parsers: Vec<Box<Parser<T>>>,
}

pub fn build<T: Clone, P: Parser<T>>(parser: &P) -> Seq<T> {
  Seq {
    parsers: vec![parser.box_clone()],
  }
}

impl<T: Clone> Seq<T> {
  pub fn and<P: Parser<T>>(mut self, parser: &P) -> Self {
    self.parsers.push(parser.box_clone());
    self
  }
}

impl<T: Clone + 'static> Parser<T> for Seq<T> {
  fn box_clone(&self) -> Box<Parser<T>> {
    let mut parsers: Vec<Box<Parser<T>>> = vec![];

    for parser in self.parsers.iter() {
      parsers.push(parser.box_clone());
    }

    Box::new(Seq { parsers: parsers })
  }

  fn parse(&self, target: &str, position: usize) -> State<T> {
    let mut result: Vec<Node<T>> = vec![];
    let mut position: usize = position;

    for parser in self.parsers.iter() {
      let parsed = parser.parse(target, position);

      if parsed.success {
        if let Some(node) = parsed.node {
          result.push(node);
        }
        position = parsed.position;
      } else {
        return State {
          success: false,
          node: None,
          position: position,
        };
      }
    }

    State {
      success: true,
      node: Some(Node {
        value: Type::Arr(result),
        kind: None,
      }),
      position: position,
    }
  }
}
