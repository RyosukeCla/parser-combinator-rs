use crate::parser::base::{Node, Parser, State, Type};

pub struct Many<T> {
  parser: Box<Parser<T>>,
}

pub fn build<T: Clone, P: Parser<T>>(parser: &P) -> Many<T> {
  Many {
    parser: parser.box_clone(),
  }
}

impl<T: Clone + 'static> Parser<T> for Many<T> {
  fn box_clone(&self) -> Box<Parser<T>> {
    Box::new(Many {
      parser: self.parser.box_clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State<T> {
    let mut result: Vec<Node<T>> = vec![];
    let mut position: usize = position;

    loop {
      let parsed = self.parser.parse(target, position);
      if parsed.success {
        if let Some(node) = parsed.node {
          result.push(node);
        }
        position = parsed.position;
      } else {
        break;
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
