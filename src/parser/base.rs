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

#[derive(Debug, Clone)]
pub struct DefaultType {}

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

pub trait ToType<T: Clone> {
  fn to_type(&self) -> Type<T>;
}

impl<T: Clone> ToType<T> for String {
  fn to_type(&self) -> Type<T> {
    Type::Str(self.clone())
  }
}

impl<T: Clone> ToType<T> for char {
  fn to_type(&self) -> Type<T> {
    Type::Char(self.clone())
  }
}

impl<T: Clone> ToType<T> for isize {
  fn to_type(&self) -> Type<T> {
    Type::Isize(self.clone())
  }
}

impl<T: Clone> ToType<T> for usize {
  fn to_type(&self) -> Type<T> {
    Type::Usize(self.clone())
  }
}

impl<T: Clone> ToType<T> for u8 {
  fn to_type(&self) -> Type<T> {
    Type::U8(self.clone())
  }
}

impl<T: Clone> ToType<T> for u16 {
  fn to_type(&self) -> Type<T> {
    Type::U16(self.clone())
  }
}

impl<T: Clone> ToType<T> for u32 {
  fn to_type(&self) -> Type<T> {
    Type::U32(self.clone())
  }
}

impl<T: Clone> ToType<T> for u64 {
  fn to_type(&self) -> Type<T> {
    Type::U64(self.clone())
  }
}

impl<T: Clone> ToType<T> for u128 {
  fn to_type(&self) -> Type<T> {
    Type::U128(self.clone())
  }
}

impl<T: Clone> ToType<T> for i16 {
  fn to_type(&self) -> Type<T> {
    Type::I16(self.clone())
  }
}

impl<T: Clone> ToType<T> for i32 {
  fn to_type(&self) -> Type<T> {
    Type::I32(self.clone())
  }
}

impl<T: Clone> ToType<T> for i64 {
  fn to_type(&self) -> Type<T> {
    Type::I64(self.clone())
  }
}

impl<T: Clone> ToType<T> for i128 {
  fn to_type(&self) -> Type<T> {
    Type::I128(self.clone())
  }
}

impl<T: Clone> ToType<T> for f32 {
  fn to_type(&self) -> Type<T> {
    Type::F32(self.clone())
  }
}

impl<T: Clone> ToType<T> for f64 {
  fn to_type(&self) -> Type<T> {
    Type::F64(self.clone())
  }
}

impl<T: Clone> ToType<T> for bool {
  fn to_type(&self) -> Type<T> {
    Type::Bool(self.clone())
  }
}

impl<T: Clone> ToType<T> for Vec<Node<T>> {
  fn to_type(&self) -> Type<T> {
    Type::Arr(self.clone())
  }
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
