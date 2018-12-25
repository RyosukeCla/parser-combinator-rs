use std::fmt;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct State<T: Clone> {
  pub success: bool,
  pub node: Option<Node<T>>,
  pub position: usize,
}

#[derive(Debug, Clone)]
pub struct Node<T: Clone> {
  pub value: Type<T>,
  pub kind: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Type<T: Clone> {
  Str(String),
  Char(char),
  Isize(isize),
  Usize(usize),
  U8(u8),
  U16(u16),
  U32(u32),
  U64(u64),
  U128(u128),
  I16(i16),
  I32(i32),
  I64(i64),
  I128(i128),
  F32(f32),
  F64(f64),
  Bool(bool),
  Val(T),
  Arr(Vec<Node<T>>),
}

pub trait Parser<T: Clone> {
  fn parse(&self, target: &str, position: usize) -> State<T>;
  fn box_clone(&self) -> Box<Parser<T>>;
}

fn recursive_fmt<T: Clone + Debug>(node: &Node<T>) -> String {
  let mut res = "".to_string();

  match &node.kind {
    Some(kind) => {
      res.push_str(format!("{} ", kind).as_str());
    }
    None => {}
  }

  match &node.value {
    Type::Arr(children) => {
      res.push_str("[");

      for child in children {
        res.push_str(recursive_fmt(&child).as_str());
        res.push_str(", ");
      }

      res.pop();
      res.pop();

      res.push_str("]");

      res
    }
    _ => {
      let val = format!("{:?}", node.value.clone());
      res.push_str(val.as_str());

      res
    }
  }
}

impl<T: Clone + Debug> fmt::Display for Node<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let res = recursive_fmt(self);
    write!(f, "{}", res)
  }
}
