use crate::parser::base::{Parser, State};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Lazy<K> {
  parser: Rc<RefCell<Option<Box<Parser<K>>>>>,
}

pub fn build<K: Clone>() -> Lazy<K> {
  Lazy {
    parser: Rc::new(RefCell::new(None)),
  }
}

impl<K: Clone> Lazy<K> {
  pub fn set_parser<P: Parser<K>>(self, parser: &P) -> Lazy<K> {
    {
      let mut option = self.parser.borrow_mut();
      option.replace(parser.box_clone());
    }
    self
  }
}

impl<K: Clone + 'static> Parser<K> for Lazy<K> {
  fn box_clone(&self) -> Box<Parser<K>> {
    Box::new(Lazy {
      parser: self.parser.clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State<K> {
    let parser = self.parser.borrow();
    let parser = parser.as_ref();

    match parser {
      Some(parser) => parser.parse(target, position),
      None => panic!("Set parser to lazy combinator."),
    }
  }
}
