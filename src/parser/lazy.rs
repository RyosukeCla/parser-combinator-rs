use crate::parser::base::{Parser, State};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Lazy {
  parser: Rc<RefCell<Option<Box<Parser>>>>,
}

pub fn build() -> Lazy {
  Lazy {
    parser: Rc::new(RefCell::new(None)),
  }
}

impl Lazy {
  pub fn set_parser<P: Parser>(self, parser: &P) -> Lazy {
    {
      let mut option = self.parser.borrow_mut();
      option.replace(parser.box_clone());
    }
    self
  }
}

impl Parser for Lazy {
  fn box_clone(&self) -> Box<Parser> {
    Box::new(Lazy {
      parser: self.parser.clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State {
    let parser = self.parser.borrow();
    let parser = parser.as_ref();

    match parser {
      Some(parser) => parser.parse(target, position),
      None => panic!("Set parser to lazy combinator."),
    }
  }
}
