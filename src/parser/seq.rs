use crate::parser::base::{Node, Parser, State};

pub struct Seq<K> {
  parsers: Vec<Box<Parser<K>>>,
}

pub fn build<K: Clone, P: Parser<K>>(parser: &P) -> Seq<K> {
  Seq {
    parsers: vec![parser.box_clone()],
  }
}

impl<K: Clone> Seq<K> {
  pub fn and<P: Parser<K>>(mut self, parser: &P) -> Self {
    self.parsers.push(parser.box_clone());
    self
  }
}

impl<K: Clone + 'static> Parser<K> for Seq<K> {
  fn box_clone(&self) -> Box<Parser<K>> {
    let mut parsers: Vec<Box<Parser<K>>> = vec![];

    for parser in self.parsers.iter() {
      parsers.push(parser.box_clone());
    }

    Box::new(Seq { parsers: parsers })
  }

  fn parse(&self, target: &str, position: usize) -> State<K> {
    let mut result: Vec<Node<K>> = vec![];
    let mut position: usize = position;

    for parser in self.parsers.iter() {
      let parsed = parser.parse(target, position);

      if parsed.success {
        if let Some(node) = parsed.node {
          result.push(node.clone());
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
        value: None,
        children: Some(result),
        kind: None,
      }),
      position: position,
    }
  }
}
