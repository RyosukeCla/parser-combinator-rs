extern crate parser_comb;
use parser_comb::parser::{
  char, choice, extract, flatten, kind, lazy, many, regexp, seq, token, trim, type_map, wrap,
  ParserCombinator,
};

pub fn main() {
  let space = token(" ");
  let num = kind(
    &type_map::<_, _, i32>(&trim(&regexp(r"([1-9][0-9]*|[0-9])"), &space)),
    "Num",
  );
  let operator = kind(&char("+-*/"), "Op");
  let parenthesis = lazy();
  let atom = choice(&num).or(&parenthesis);
  let expression = kind(
    &flatten(&seq(&wrap(&atom)).and(&flatten(&many(&seq(&operator).and(&atom))))),
    "Expr",
  );
  let paren_open = trim(&token("("), &space);
  let paren_close = trim(&token(")"), &space);

  parenthesis.set_parser(&extract(
    &seq(&paren_open).and(&expression).and(&paren_close),
    1, // extract expression
  ));

  let parser: ParserCombinator = ParserCombinator::new(&expression);

  let targets = vec![
    "10+20-(3+1-(4))",
    "hoge",
    "1+2-(3+1",
    "0-3+(((3)))",
    "1 + 2 + ( 20 + 3 ) / (30 - 20) * (10 - 5)",
    "1 + 2 - (3 * 4) / (5)",
  ];

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
