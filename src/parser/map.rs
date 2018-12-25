use crate::parser::base::{Node, Parser, State};
use std::rc::Rc;

pub struct Map<K: Clone> {
  parser: Box<Parser<K>>,
  mapper: Rc<Box<Fn(Node<K>) -> Node<K>>>,
}

pub fn build<K: Clone, P: Parser<K>>(parser: &P, mapper: Box<Fn(Node<K>) -> Node<K>>) -> Map<K> {
  Map {
    parser: parser.box_clone(),
    mapper: Rc::new(mapper),
  }
}

impl<K: Clone + 'static> Parser<K> for Map<K> {
  fn box_clone(&self) -> Box<Parser<K>> {
    Box::new(Map {
      parser: self.parser.box_clone(),
      mapper: self.mapper.clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State<K> {
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
