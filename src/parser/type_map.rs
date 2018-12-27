use crate::parser::base::{Node, Parser, ToType, Type};
use crate::parser::map;

/**
 *  TypeMap
 *  Type::Str -> another Type
 */
pub fn build<T: Clone, P: Parser<T>, O: ToType<T>>(parser: &P) -> map::Map<T>
where
  O: std::str::FromStr + std::fmt::Debug,
{
  map::build(parser, move |node| {
    let value = match node.value {
      Type::Str(value) => match value.parse::<O>() {
        Ok(value) => value.to_type(),
        Err(_) => panic!("Error!"),
      },
      _ => panic!("Couldn't parse value: node.value is not Type::Str"),
    };

    Node {
      value,
      kind: node.kind,
    }
  })
}
