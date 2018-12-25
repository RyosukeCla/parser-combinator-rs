use crate::parser::base::{Parser, State};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Lazy<T> {
  parser: Rc<RefCell<Option<Box<Parser<T>>>>>,
}

pub fn build<T: Clone>() -> Lazy<T> {
  Lazy {
    parser: Rc::new(RefCell::new(None)),
  }
}

impl<T: Clone> Lazy<T> {
  pub fn set_parser<P: Parser<T>>(self, parser: &P) -> Lazy<T> {
    {
      let mut option = self.parser.borrow_mut();
      option.replace(parser.box_clone());
    }
    self
  }
}

impl<T: Clone + 'static> Parser<T> for Lazy<T> {
  fn box_clone(&self) -> Box<Parser<T>> {
    Box::new(Lazy {
      parser: self.parser.clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State<T> {
    let parser = self.parser.borrow();
    let parser = parser.as_ref();

    match parser {
      Some(parser) => parser.parse(target, position),
      None => panic!("Set parser to lazy combinator."),
    }
  }
}
