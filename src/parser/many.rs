use crate::parser::base::{Node, Parser, State};

pub struct Many {
  parser: Box<Parser>,
}

pub fn build<P: Parser>(parser: &P) -> Many {
  Many {
    parser: parser.box_clone(),
  }
}

impl Parser for Many {
  fn box_clone(&self) -> Box<Parser> {
    Box::new(Many {
      parser: self.parser.box_clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State {
    let mut result: Vec<Node> = vec![];
    let mut position: usize = position;

    loop {
      let parsed = self.parser.parse(target, position);
      if parsed.success {
        if let Some(node) = &parsed.node {
          result.push(node.clone());
        }
        position = parsed.position;
      } else {
        break;
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
