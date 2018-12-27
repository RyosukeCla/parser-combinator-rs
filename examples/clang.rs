extern crate parser_comb;
use parser_comb::parser::map::Map;
use parser_comb::parser::{
  char, choice, debug_parse, filter, kind, many, map, regexp, seq, token, trim, type_map, unwrap,
  Node, Parser, ParserCombinator, Type,
};

#[derive(Clone, Debug)]
enum ExtendedType {}

const CODE_1: &str = "int int_var=32;";

const CODE_2: &str = r#"
int test=19;

int main(){
  int int_var = 32;
  return int_var;
}
"#;

const CODE_3: &str = r#"
int_var = 1;
int_var = 2 ;
"#;

fn kind_ignore<T, P>(parser: &P, ignore: &'static str) -> Map<T>
where
  T: Clone,
  P: Parser<T>,
{
  filter(parser, move |node| match &node.kind {
    Some(kind) => kind != ignore,
    _ => true,
  })
}

pub fn main() {
  let space = token(" ");
  let new_line = token("\n");
  let tab = token("\t");
  let whitespace = kind(&choice(&space).or(&new_line).or(&tab), "WS");
  let whitespaces = kind(&many(&whitespace), "WS");
  let equal = token("=");
  let identifier = regexp(r"([a-zA-Z_][a-zA-Z0-9_]*)");
  let semicolon = token(";");
  let num = regexp(r"([1-9][0-9]*|[0-9])");

  let int_def = token("int");
  let var_int_def = kind(
    &kind_ignore(
      &seq(&int_def)
        .and(&whitespace)
        .and(&identifier)
        .and(&whitespaces)
        .and(&equal)
        .and(&whitespaces)
        .and(&num)
        .and(&whitespaces)
        .and(&semicolon),
      "WS",
    ),
    "VAR_INT_DEF",
  );

  let int_substitute = kind(
    &kind_ignore(
      &seq(&identifier)
        .and(&whitespaces)
        .and(&equal)
        .and(&whitespaces)
        .and(&num)
        .and(&whitespaces)
        .and(&semicolon),
      "WS",
    ),
    "INT_SUBSTITUTE",
  );

  let return_int = seq(&token("return"))
    .and(&whitespace)
    .and(&choice(&num).or(&identifier))
    .and(&whitespaces)
    .and(&semicolon);

  let sentence = choice(&var_int_def)
    .or(&int_substitute)
    .or(&return_int)
    .or(&whitespace);
  let sentences = kind_ignore(&many(&sentence), "WS");

  let int_func_def = kind(
    &seq(&int_def)
      .and(&whitespace)
      .and(&identifier)
      .and(&token("(){"))
      .and(&sentences)
      .and(&token("}")),
    "INT_FUNC_DEF",
  );

  let parser = kind_ignore(
    &many(&choice(&sentence).or(&int_func_def).or(&new_line)),
    "WS",
  );
  let parser: ParserCombinator<ExtendedType> = ParserCombinator::new(&parser);

  let targets = vec![CODE_1, CODE_2, CODE_3];
  for target in targets {
    println!("[In]:\n   {}\n", target);
    match parser.parse(target) {
      Ok(res) => {
        println!("[Out]:\n   {}\n", res);
      }
      Err(message) => {
        println!("[Out]:\n   {}\n", message);
      }
    }
  }
}
