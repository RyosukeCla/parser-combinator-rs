use crate::parser::base::{Node, Parser, State};
use std::rc::Rc;

pub struct Map {
  parser: Box<Parser>,
  mapper: Rc<Box<Fn(Node) -> Node>>,
}

pub fn build<P: Parser>(parser: &P, mapper: Box<Fn(Node) -> Node>) -> Map {
  Map {
    parser: parser.box_clone(),
    mapper: Rc::new(mapper),
  }
}

impl Parser for Map {
  fn box_clone(&self) -> Box<Parser> {
    Box::new(Map {
      parser: self.parser.box_clone(),
      mapper: self.mapper.clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State {
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
