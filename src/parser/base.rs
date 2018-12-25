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
