use crate::parser::base::{Node, Parser, State};

pub struct Token {
  token: String,
  len: usize,
}

pub fn build(token: &str) -> Token {
  Token {
    token: token.to_string(),
    len: token.len(),
  }
}

impl<K: Clone> Parser<K> for Token {
  fn box_clone(&self) -> Box<Parser<K>> {
    Box::new(Token {
      token: self.token.clone(),
      len: self.len,
    })
  }

  fn parse(&self, target: &str, position: usize) -> State<K> {
    let next_position = match target.len() {
      x if x < position + self.len => x,
      _ => position + self.len,
    };

    if position == next_position {
      return State {
        success: false,
        node: None,
        position: position,
      };
    }

    match &target[position..next_position] {
      x if x == self.token => State {
        success: true,
        node: Some(Node {
          value: Some(self.token.clone()),
          children: None,
          kind: None,
        }),
        position: next_position,
      },
      _ => State {
        success: false,
        node: None,
        position: position,
      },
    }
  }
}
