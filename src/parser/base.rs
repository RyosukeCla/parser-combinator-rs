use std::fmt;

#[derive(Debug, Clone)]
pub struct State {
  pub success: bool,
  pub node: Option<Node>,
  pub position: usize,
}

#[derive(Debug, Clone)]
pub struct Node {
  pub value: Option<String>,
  pub children: Option<Vec<Node>>,
}

pub trait Parser {
  fn parse(&self, target: &str, position: usize) -> State;
  fn box_clone(&self) -> Box<Parser>;
}

fn recursive_fmt(node: &Node) -> String {
  let mut res = "".to_string();

  match &node.value {
    Some(value) => {
      return value.clone();
    }
    None => {}
  }

  match &node.children {
    Some(children) => {
      res.push_str("[");

      for child in children {
        res.push_str(recursive_fmt(&child).as_str());
        res.push_str(", ");
      }

      res.pop();
      res.pop();

      res.push_str("]");

      return res;
    }
    None => {}
  }

  res
}

impl fmt::Display for Node {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let res = recursive_fmt(self);
    write!(f, "{}", res)
  }
}
