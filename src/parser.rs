use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/**
 * Base
 */
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

/**
 * Token
 */
pub struct Token {
  token: String,
  len: usize,
}

impl Token {
  pub fn new(token: &str) -> Token {
    Token {
      token: token.to_string(),
      len: token.len(),
    }
  }
}

impl Parser for Token {
  fn box_clone(&self) -> Box<Parser> {
    Box::new(Token {
      token: self.token.clone(),
      len: self.len,
    })
  }

  fn parse(&self, target: &str, position: usize) -> State {
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

/**
 * Many
 */
pub struct Many {
  parser: Box<Parser>,
}

impl Many {
  pub fn new<P: Parser>(parser: &P) -> Many {
    Many {
      parser: parser.box_clone(),
    }
  }
}

impl Parser for Many {
  fn box_clone(&self) -> Box<Parser> {
    Box::new(Many {
      parser: self.parser.box_clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State {
    let mut result: Vec<Node> = vec![];
    let mut position: usize = position;

    loop {
      let parsed = self.parser.parse(target, position);
      if parsed.success {
        if let Some(node) = &parsed.node {
          result.push(node.clone());
        }
        position = parsed.position;
      } else {
        break;
      }
    }

    State {
      success: true,
      node: Some(Node {
        value: None,
        children: Some(result),
      }),
      position: position,
    }
  }
}

/**
 * Seq
 */
pub struct Seq {
  parsers: Vec<Box<Parser>>,
}

impl Seq {
  pub fn new<P: Parser>(parser: &P) -> Seq {
    Seq {
      parsers: vec![parser.box_clone()],
    }
  }

  pub fn and<P: Parser>(mut self, parser: &P) -> Self {
    self.parsers.push(parser.box_clone());
    self
  }
}

impl Parser for Seq {
  fn box_clone(&self) -> Box<Parser> {
    let mut parsers: Vec<Box<Parser>> = vec![];

    for parser in self.parsers.iter() {
      parsers.push(parser.box_clone());
    }

    Box::new(Seq { parsers: parsers })
  }

  fn parse(&self, target: &str, position: usize) -> State {
    let mut result: Vec<Node> = vec![];
    let mut position: usize = position;

    for parser in self.parsers.iter() {
      let parsed = parser.parse(target, position);

      if parsed.success {
        if let Some(node) = parsed.node {
          result.push(node.clone());
        }
        position = parsed.position;
      } else {
        return State {
          success: false,
          node: None,
          position: position,
        };
      }
    }

    State {
      success: true,
      node: Some(Node {
        value: None,
        children: Some(result),
      }),
      position: position,
    }
  }
}

/**
 * Choice
 */
pub struct Choice {
  parsers: Vec<Box<Parser>>,
}

impl Choice {
  pub fn new<P: Parser>(parser: &P) -> Choice {
    Choice {
      parsers: vec![parser.box_clone()],
    }
  }

  pub fn or<P: Parser>(mut self, parser: &P) -> Self {
    self.parsers.push(parser.box_clone());
    self
  }
}

impl Parser for Choice {
  fn box_clone(&self) -> Box<Parser> {
    let mut parsers: Vec<Box<Parser>> = vec![];

    for parser in self.parsers.iter() {
      parsers.push(parser.box_clone());
    }

    Box::new(Choice { parsers: parsers })
  }

  fn parse(&self, target: &str, position: usize) -> State {
    for parser in self.parsers.iter() {
      let parsed = parser.parse(target, position);

      if parsed.success {
        return parsed;
      }
    }

    State {
      success: false,
      node: None,
      position: position,
    }
  }
}

/**
 * Option
 */
pub struct Opt {
  parser: Box<Parser>,
}

impl Opt {
  pub fn new<P: Parser>(parser: &P) -> Opt {
    Opt {
      parser: parser.box_clone(),
    }
  }
}

impl Parser for Opt {
  fn box_clone(&self) -> Box<Parser> {
    Box::new(Opt {
      parser: self.parser.box_clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State {
    let parsed = self.parser.parse(target, position);
    if parsed.success {
      parsed
    } else {
      State {
        success: true,
        node: None,
        position: position,
      }
    }
  }
}

/**
 * RegExp
 */
pub struct RegExp {
  regex: Regex,
}

impl RegExp {
  pub fn new(regex: &str) -> RegExp {
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

/**
 * Map
 */
pub struct Map {
  parser: Box<Parser>,
  mapper: Rc<Box<Fn(Node) -> Node>>,
}

impl Map {
  pub fn new<P: Parser>(parser: &P, mapper: Box<Fn(Node) -> Node>) -> Map {
    Map {
      parser: parser.box_clone(),
      mapper: Rc::new(mapper),
    }
  }
}

impl Parser for Map {
  fn box_clone(&self) -> Box<Parser> {
    Box::new(Map {
      parser: self.parser.box_clone(),
      mapper: self.mapper.clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State {
    let parsed = self.parser.parse(target, position);

    if parsed.success {
      State {
        success: parsed.success,
        node: Some((self.mapper)(parsed.node.unwrap())),
        position: parsed.position,
      }
    } else {
      parsed
    }
  }
}

/**
 * Char
 */
pub struct Char {
  dict: Rc<HashMap<String, String>>,
}

impl Char {
  pub fn new(chars: &str) -> Char {
    let mut dict = HashMap::new();
    for c in chars.chars() {
      let s = c.to_string();
      dict.insert(s.clone(), s.clone());
    }

    Char {
      dict: Rc::new(dict),
    }
  }
}

impl Parser for Char {
  fn box_clone(&self) -> Box<Parser> {
    Box::new(Char {
      dict: self.dict.clone(),
    })
  }

  fn parse(&self, target: &str, position: usize) -> State {
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
          value: Some(s.clone()),
          children: None,
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

/**
 * Lazy: for recursive combinator
 */
pub struct Lazy {
  parser: Rc<RefCell<Option<Box<Parser>>>>,
}

impl Lazy {
  pub fn new() -> Lazy {
    Lazy {
      parser: Rc::new(RefCell::new(None)),
    }
  }

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
