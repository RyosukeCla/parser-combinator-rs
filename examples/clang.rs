#[macro_use]
extern crate parser_comb;
use parser_comb::parser::map::Map;
use parser_comb::parser::{
  char, choice, debug_parse, exclude_empty, extract, filter, flatten, identity_map, kind,
  kind_ignore, lazy, many, many1, map, opt, regexp, sep, seq, token, trim, type_map, unwrap, wrap,
  Node, Parser, ParserCombinator, Type,
};

#[derive(Clone, Debug)]
enum ExtendedType {}

const CODE: &str = r#"
int int_var = 1;

int test(int a, int b) {
  int var = 10;
  int_var = (a + b) * int_var + var;
  return int_var;
}

int main() {
  return test(100, int_var + test(10, 200));
  test(100, 100);
  test();
}
"#;

const DELIMITER: &str = "DELIMITER";
const VAR_DECL: &str = "VAR_DECL";

pub fn main() {
  let space = token(" ");
  let new_line = token("\n");
  let tab = token("\t");
  let whitespace = kind(&choice(&space).or(&new_line).or(&tab), DELIMITER);
  let ws = kind(&choice(&space).or(&new_line).or(&tab), DELIMITER);
  let ws_1 = kind(&many1(&ws), DELIMITER);
  let ws_0 = kind(&many(&ws), DELIMITER);
  let whitespaces = kind(&many(&whitespace), DELIMITER);
  let semicolon = kind(&token(";"), DELIMITER);
  let identifier = regexp(r"([a-zA-Z_][a-zA-Z0-9_]*)");
  let num = regexp(r"([1-9][0-9]*|[0-9])");
  let equal = token("=");

  // EXPR STMT
  let paren_block = lazy();
  let operation = kind(&char("=+-*/"), "OPERATOR");
  let call_expr = lazy();
  let atom = choice(&call_expr).or(&identifier).or(&num).or(&paren_block);
  let binary_op = lazy();
  let binary_op_cloned = identity_map(&binary_op);
  paren_block.set_parser(&extract(
    &seq(&trim(&token("("), &ws))
      .and(&choice(&binary_op_cloned).or(&atom))
      .and(&trim(&token(")"), &ws)),
    1,
  ));
  binary_op.set_parser(&kind(
    &kind_ignore(
      &seq(&atom)
        .and(&ws_0)
        .and(&operation)
        .and(&ws_0)
        .and(&choice(&binary_op_cloned).or(&atom)),
      DELIMITER,
    ),
    "BINARY_OP",
  ));
  let expr_stmt = extract(&seq(&binary_op_cloned).and(&whitespaces).and(&semicolon), 0);

  // CALL EXPR STMT
  let call_expr_stmt = kind(
    &kind_ignore(&seq(&call_expr).and(&ws_0).and(&semicolon), DELIMITER),
    "CALL_EXPR_STMT",
  );

  // CALL EXPR
  // (a, b, c) -> [a, b, c]
  let params_for_call = kind_ignore(
    &seq(&kind(&trim(&token("("), &ws), DELIMITER))
      .and(&opt(&sep(
        &choice(&binary_op_cloned).or(&atom),
        &trim(&token(","), &ws),
      )))
      .and(&kind(&trim(&token(")"), &ws), DELIMITER)),
    DELIMITER,
  );
  call_expr.set_parser(&kind(
    &flatten(&exclude_empty(&kind_ignore(
      &seq(&wrap(&identifier)).and(&params_for_call),
      DELIMITER,
    ))),
    "CALL_EXPR",
  ));

  // VAR DECL
  let int_type = token("int");
  let float_type = token("float");
  let types = choice(&int_type).or(&float_type);
  let var_decl = kind(
    &kind_ignore(
      &seq(&types)
        .and(&ws_1)
        .and(&identifier)
        .and(&ws_0)
        .and(&equal)
        .and(&ws_0)
        .and(&num)
        .and(&ws_0)
        .and(&semicolon),
      DELIMITER,
    ),
    VAR_DECL,
  );

  // RETURN STMT
  let return_symbol = token("return");
  let return_stmt = kind(
    &kind_ignore(
      &choice(&seq(&return_symbol).and(&semicolon)).or(
        &seq(&return_symbol)
          .and(&ws_1)
          .and(&choice(&binary_op_cloned).or(&atom))
          .and(&ws_0)
          .and(&semicolon),
      ),
      DELIMITER,
    ),
    "RETURN_STMT",
  );

  // PARAM VAR DECL: can be null
  let param_var_decl = kind(
    &kind_ignore(&seq(&types).and(&ws_1).and(&identifier), DELIMITER),
    "PARAM_VAR_DECL",
  );
  let param_var_decl = kind_ignore(
    &seq(&kind(&trim(&token("("), &ws), DELIMITER))
      .and(&opt(&sep(&param_var_decl, &trim(&token(","), &ws))))
      .and(&kind(&trim(&token(")"), &ws), DELIMITER)),
    DELIMITER,
  );

  // COMPOUND STMT
  let compound_stmt = kind(
    &kind_ignore(
      &many(
        &choice(&ws)
          .or(&var_decl)
          .or(&expr_stmt)
          .or(&return_stmt)
          .or(&call_expr_stmt),
      ),
      DELIMITER,
    ),
    "COMPOUND_STML",
  );

  // FUNC DECL
  let func_decl = kind(
    &exclude_empty(&kind_ignore(
      &seq(&types)
        .and(&ws_1)
        .and(&identifier)
        .and(&param_var_decl)
        .and(&extract(
          &seq(&trim(&token("{"), &ws))
            .and(&compound_stmt)
            .and(&trim(&token("}"), &ws)),
          1,
        )),
      DELIMITER,
    )),
    "FUNC_DECL",
  );

  /*
   * STMT
   */
  let stmt = choice(&var_decl).or(&func_decl).or(&ws_1);

  let parser = kind_ignore(&many(&stmt), DELIMITER);
  let parser: ParserCombinator<ExtendedType> = ParserCombinator::new(&parser);

  let targets = vec![CODE];
  for target in targets {
    println!("[In]:\n   {}\n", target);
    match parser.parse(target) {
      Ok(res) => {
        println!("[Out]:\n{}\n", res);
      }
      Err(message) => {
        println!("[Out]:\n{}\n", message);
      }
    }
  }
}
