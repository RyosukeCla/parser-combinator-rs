use crate::parser::base::{Node, Parser, State, Type};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Char {
  dict: Rc<HashMap<String, String>>,
}

pub fn build(chars: &str) -> Char {
  let mut dict = HashMap::new();
  for c in chars.chars() {
    let s = c.to_string();
    dict.insert(s.clone(), s.clone());
  }

  Char {
    dict: Rc::new(dict),
  }
}

impl<T: Clone> Parser<T> for Char {
  fn box_clone(&self) -> Box<Parser<T>> {
    Box::new(Char {
      dict: self.dict.clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State<T> {
    let next_position = if target.len() == position {
      position
    } else {
      position + 1
    };

    let c = &target[position..next_position];
    let c = c.to_string();

    match self.dict.get(c.as_str()) {
      Some(s) => State {
        success: true,
        node: Some(Node {
          value: Type::Str(s.clone()),
          kind: None,
        }),
        position: next_position,
      },
      None => State {
        success: false,
        node: None,
        position: position,
      },
    }
  }
}
