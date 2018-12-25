use crate::parser::base::{Node, Parser, State};

pub struct Seq {
  parsers: Vec<Box<Parser>>,
}

pub fn build<P: Parser>(parser: &P) -> Seq {
  Seq {
    parsers: vec![parser.box_clone()],
  }
}

impl Seq {
  pub fn and<P: Parser>(mut self, parser: &P) -> Self {
    self.parsers.push(parser.box_clone());
    self
  }
}

impl Parser for Seq {
  fn box_clone(&self) -> Box<Parser> {
    let mut parsers: Vec<Box<Parser>> = vec![];

    for parser in self.parsers.iter() {
      parsers.push(parser.box_clone());
    }

    Box::new(Seq { parsers: parsers })
  }

  fn parse(&self, target: &str, position: usize) -> State {
    let mut result: Vec<Node> = vec![];
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
      }),
      position: position,
    }
  }
}
