use crate::parser::base::{Node, Parser, State};
use std::rc::Rc;

pub struct Map<T: Clone> {
  parser: Box<Parser<T>>,
  mapper: Rc<Box<Fn(Node<T>) -> Node<T>>>,
}

pub fn build<T, P, F>(parser: &P, mapper: F) -> Map<T>
where
  T: Clone,
  P: Parser<T>,
  F: Fn(Node<T>) -> Node<T> + 'static,
{
  Map {
    parser: parser.box_clone(),
    mapper: Rc::new(Box::new(mapper)),
  }
}

impl<T: Clone + 'static> Parser<T> for Map<T> {
  fn box_clone(&self) -> Box<Parser<T>> {
    Box::new(Map {
      parser: self.parser.box_clone(),
      mapper: self.mapper.clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State<T> {
    let parsed = self.parser.parse(target, position);

    if parsed.success {
      State {
        success: parsed.success,
        node: Some((self.mapper)(parsed.node.unwrap())),
        position: parsed.position,
      }
    } else {
      parsed
    }
  }
}
