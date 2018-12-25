use crate::parser::base::{Node, Parser, State};
use regex::Regex;

pub struct RegExp {
  regex: Regex,
}

pub fn build(regex: &str) -> RegExp {
  let fixed_regex = format!("^{}", regex);
  let fixed_regex = match &regex[0..1] {
    "^" => regex,
    _ => fixed_regex.as_str(),
  };

  let reg = match Regex::new(fixed_regex) {
    Ok(reg) => reg,
    Err(_) => panic!("Regex Error"),
  };

  RegExp { regex: reg }
}

impl Parser for RegExp {
  fn box_clone(&self) -> Box<Parser> {
    Box::new(RegExp {
      regex: self.regex.clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State {
    let sliced = &target[position..];
    match self.regex.captures(sliced) {
      Some(caps) => {
        let res = caps.get(0).unwrap().as_str().to_string();
        let len = res.len();

        State {
          success: true,
          node: Some(Node {
            value: Some(res),
            children: None,
          }),
          position: position + len,
        }
      }
      None => State {
        success: false,
        node: None,
        position: position,
      },
    }
  }
}
