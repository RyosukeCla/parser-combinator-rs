use std::fmt;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct State<K: Clone> {
  pub success: bool,
  pub node: Option<Node<K>>,
  pub position: usize,
}

#[derive(Debug, Clone)]
pub struct Node<K: Clone> {
  pub value: Option<String>,
  pub children: Option<Vec<Node<K>>>,
  pub kind: Option<K>,
}

pub trait Parser<K: Clone> {
  fn parse(&self, target: &str, position: usize) -> State<K>;
  fn box_clone(&self) -> Box<Parser<K>>;
}

fn recursive_fmt<K: Clone + Debug>(node: &Node<K>) -> String {
  let mut res = "".to_string();

  match &node.kind {
    Some(kind) => {
      res.push_str(format!("{:?} ", kind).as_str());
    }
    None => {}
  }

  match &node.value {
    Some(value) => {
      res.push_str(value.clone().as_str());
      return res;
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

impl<K: Clone + Debug> fmt::Display for Node<K> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let res = recursive_fmt(self);
    write!(f, "{}", res)
  }
}
