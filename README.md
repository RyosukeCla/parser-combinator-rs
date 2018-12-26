# Parser Combinator - rust implementation

Under construction..

## Examples

### Expression

```rust
use crate::parser::{
  char, choice, extract_map, flatten_map, kind, lazy, many, map, regexp, seq, token, trim,
  type_map, wrap_map, Node, ParserCombinator, Type,
};

fn main() {
  let space = token(" ");
  let num = kind(
    &type_map::<_, _, i32>(&trim(&regexp(r"([1-9][0-9]*|[0-9])"), &space)),
    "Num",
  );
  let operator = kind(&char("+-*/"), "Op");
  let parenthesis = lazy();
  let atom = choice(&num).or(&parenthesis);
  let expression = kind(
    &flatten_map(&seq(&wrap_map(&atom)).and(&flatten_map(&many(&seq(&operator).and(&atom))))),
    "Expr",
  );
  let paren_open = trim(&token("("), &space);
  let paren_close = trim(&token(")"), &space);

  parenthesis.set_parser(&extract_map(
    &seq(&paren_open).and(&expression).and(&paren_close),
    1, // extract expression
  ));

  let parser: ParserCombinator = ParserCombinator::new(&expression);
  let target = "1 + 2 - (3 * 4) / (5)";
  println!("[In]:\n{}\n", target);
  println!("[Out]:\n{}\n", parser.parse(target).unwrap()); // simple print
}
```

```
[In]:
1 + 2 - (3 * 4) / (5)

[Out]:
Expr [Num I32(1), Op Str("+"), Num I32(2), Op Str("-"), Expr [Num I32(3), Op Str("*"), Num I32(4)], Op Str("/"), Expr [Num I32(5)]]
```

### Complex Number

```rust
use crate::parser::{
  map, regexp, seq, token, type_map, Node, ParserCombinator, Type,
};

#[derive(Clone, Debug)]
enum ExtendedType {
  Complex32(i32, i32),
}

fn main() {
  let num = type_map::<_, _, i32>(&regexp(r"([1-9][0-9]*|[0-9])"));
  let plus = token("+");
  let imaginary = token("i");
  let expr = seq(&num).and(&plus).and(&num).and(&imaginary);
  let complex = map(
    &expr,
    Box::new(|node| match node.value {
      Type::Arr(children) => {
        let re = match children[0].value {
          Type::I32(re) => re,
          _ => panic!("err"),
        };
        let im = match children[2].value {
          Type::I32(re) => re,
          _ => panic!("err"),
        };

        Node {
          value: Type::Val(ExtendedType::Complex32(re, im)),
          kind: None,
        }
      }
      _ => panic!("Error"),
    }),
  );

  let parser: ParserCombinator<ExtendedType> = ParserCombinator::new(&complex);

  let target = "100+100i";
  println!("[In]:\n{}\n", target);
  println!("[Out]:\n{:#?}\n", parser.parse(target).unwrap()); // detail print
}
```

```
[In]:
100+100i

[Out]:
Node {
    value: Val(
        Complex32(
            100,
            100
        )
    ),
    kind: None
}
```

## Basic

### parse

```rust
use crate::parser::{parse, DefaultType};
let parser = // ...
let target = "target &str";
parse::<DefaultType, _>(&parser, target); // -> Result<Node, String>
```

or

```rust
use crate::parser::ParserCombinator;
let parser = // ...
let parser = ParserCombinator::new(&parser);
let target = "target &str";
parser.parse(target); // -> Result<Node, String>
```

### custom type

Customize parsed data type.

```rust
use crate::parser::ParserCombinator;

#[derive(Clone, Debug)]
enum ExtendedType {
  ComplexI32(i32, i32),
}

let parser = // ...
let parser: ParserCombinator<ExtendedType> = ParserCombinator::new(&parser);
```

### Struct and Trait

```rust
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
```

## Combinators

### Kind

Grant label

```rust
let num = kind(&RegExp(r"([1-9][0-9]*|[0-9])"), "Num");
println!("{}", parse(&num, "100").unwrap());
// Num 100
```

### Token

```rust
let token = token("token");
println!("{}", parse(&token, "token").unwrap());
// token
```

### Char

```rust
let operator = char("+-");
println!("{}", parse(&operator, "+").unwrap());
// +
println!("{}", parse(&operator, "-").unwrap());
// -
```

### Regex

```rust
let num = regexp(r"([1-9][0-9]*|[0-9])");
println!("{}", parse(&number, "12345").unwrap());
// 12345
```

### Sequence

```rust
let seq = seq(&token("a")).and(&token("b"));
println!("{}", parse(&seq, "ab").unwrap());
// [a, b]
```

### Many

```rust
let many = many(&token("a"));
println!("{}", parse(&many, "aaaaa").unwrap());
// [a, a, a, a, a]
```

### Option

```js
/(option)?/;
```

```rust
let option = opt(&token("aaa"));
println!("{}", parse(&option, "aaa").unwrap());
// aaa
println!("{}", parse(&option, "").unwrap());
//
```

### Choice

```rust
let choice = choice(&token("a")).or(&token("b"));
println!("{}", parse(&choice, "a").unwrap());
// a
println!("{}", parse(&choice, "b").unwrap());
// b
```

### Lazy

Lazy initialized parser.
It is useful for making recursive parser.

```rust
let lazy = lazy();
// define parsers
lazy.set_parser(&parser);
```

### Map

map node to new node.

```rust
// map a to b
let map = map(
  &token("a"),
  Box::new(|node| {
    Node {
      value: Type::Str("b".to_string()),
      kind: None,
    }
  })
);
println!("{}", parse(&map, "a").unwrap());
// b
```

### Type Map

map str to specific type.

```
TypeMap(Type::Str) => Type::I32
```

```rust
let toI32 = type_map::<_, _, i32>(&parser);
let toI64 = type_map::<_, _, i64>(&parser);
```

### Extract Map

Extract an element from elements.

```
ExtractMap([ a, b, c ], 1) = b
```

```rust
let extraction = extract_map(&parser, extraction_index);
```

### Flatten Map

Flatten elements in elements.

```
FlattenMap(
  [
    [a1, b1, c1, ...],
    [a2, b2, c2, ...]
  ]
) = [a1, b1, c1, ..., a2, b2, c2, ...]
```

```rust
let flatten = flatten_map(&parser);
```

### Wrap Map

Wrap elements.

```
WrapMap([a, b, c, ...]) = [[a, b, c, ...]]
```

```rust
let wrap = wrap_map(&parser);
```

### Unwrap Map

Unwrap element.

```
WrapMap([a]) = a
```

```rust
let unwrap = unwrap_map(&parser);
```

### Trim

Triming "a" by "-" yields: "--a--" -> "a"

```rust
let trim = trim(&parser, &by);

// eg
let trim = trim(&token("a"), &token("-"));
```
