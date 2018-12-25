use crate::parser::base::{Node, Parser, State};

pub struct Many<K> {
  parser: Box<Parser<K>>,
}

pub fn build<K: Clone, P: Parser<K>>(parser: &P) -> Many<K> {
  Many {
    parser: parser.box_clone(),
  }
}

impl<K: Clone + 'static> Parser<K> for Many<K> {
  fn box_clone(&self) -> Box<Parser<K>> {
    Box::new(Many {
      parser: self.parser.box_clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State<K> {
    let mut result: Vec<Node<K>> = vec![];
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
        kind: None,
      }),
      position: position,
    }
  }
}
