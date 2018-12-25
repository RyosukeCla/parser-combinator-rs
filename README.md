# Parser Combinator - rust implementation

Under construction..

## Usage

```rust
use parser::{
    parse, Char, Choice, ExtractMap, FlattenMap, Kind, Lazy, Many, RegExp, Seq, Token, WrapMap,
};

#[derive(Clone, Debug)]
enum MyType {}

#[derive(Clone, Debug)]
const NUM: &str = "Num";
const OP: &str = "Op";
const EXPR: &str = "Expr";


// Expression Parser
pub fn main() {
    let space = Token(" ");
    let num = Kind( // grand Num Label
        &TypeMap::<_, _, i32>(&Trim(&RegExp(r"([1-9][0-9]*|[0-9])"), &space)), // mapped to i32
        NUM,
    );
    let operator = Kind(&Char("+-"), OP); // grand Op label
    let parenthesis = Lazy::<MyType>(); // lazy initialized parser
    let atom = Choice(&num).or(&parenthesis);
    let expression =
        FlattenMap(&Seq(&WrapMap(&atom)).and(&FlattenMap(&Many(&Seq(&operator).and(&atom)))));
    let paren_open = Trim(&Token("("), &space); // (
    let paren_close = Trim(&Token(")"), &space); // )
    parenthesis.set_parser(&ExtractMap(
        &Seq(&paren_open).and(&expression).and(&paren_close), // ( expre )
        1, // extract expression
    ));

    let parser = Kind(&expression, EXPR); // grant Expr label

    let target = "1 + 2 + ( 20 + 3 )";

    println!("[In]:\n   {}\n", target);
    match parse(&parser, target) {
        Ok(res) => {
            println!("[Out]:\n   {}\n", res);
        }
        Err(message) => {
            println!("[Out]:\n   {}\n", message);
        }
    }
}
```

```
[In]:
   1 + 2 + ( 20 + 3 )

[Out]:
   Expr [Num I32(1), Op Str("+"), Num I32(2), Op Str("+"), [Num I32(20), Op Str("+"), Num I32(3)]]
```

## Basic

### parse

```rust
let parser = // ...
let target = "target &str";
parse(&parser, target); // -> Result<Node, String>
```

or

```rust
let parser = // ...
let target = "target &str";
let position = 0;
parser.parse(target, position); // -> State
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
let num = Kind(&RegExp(r"([1-9][0-9]*|[0-9])"), "Num");
println!("{}", parse(&num, "100").unwrap());
// Num 100
```

### Token

```rust
let token = Token("token");
println!("{}", parse(&token, "token").unwrap());
// token
```

### Char

```rust
let operator = Char("+-");
println!("{}", parse(&operator, "+").unwrap());
// +
println!("{}", parse(&operator, "-").unwrap());
// -
```

### Regex

```rust
let num = RegExp(r"([1-9][0-9]*|[0-9])");
println!("{}", parse(&number, "12345").unwrap());
// 12345
```

### Sequence

```rust
let seq = Seq(&Token("a")).and(&Token("b"));
println!("{}", parse(&seq, "ab").unwrap());
// [a, b]
```

### Many

```rust
let many = Many(&Token("a"));
println!("{}", parse(&many, "aaaaa").unwrap());
// [a, a, a, a, a]
```

### Option

```js
/(option)?/;
```

```rust
let option = Opt(&Token("aaa"));
println!("{}", parse(&option, "aaa").unwrap());
// aaa
println!("{}", parse(&option, "").unwrap());
//
```

### Choice

```rust
let choice = Choice(&Token("a")).or(&Token("b"));
println!("{}", parse(&choice, "a").unwrap());
// a
println!("{}", parse(&choice, "b").unwrap());
// b
```

### Lazy

Lazy initialized parser.
It is useful for making recursive parser.

```rust
let lazy = Lazy();
// define parsers
lazy.set_parser(&parser);
```

### Map

map node to new node.

```rust
// map a to b
let map = Map(
  &Token("a"),
  Box::new(|node| {
    Node {
      value: Some("b".to_string()),
      children: None,
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
let toI32 = TypeMap::<_, _, i32>(&parser);
```

### Extract Map

Extract an element from elements.

```
ExtractMap([ a, b, c ], 1) = b
```

```rust
let extraction = ExtractMap(&parser, extraction_index);
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
let flatten = FlattenMap(&parser);
```

### Wrap Map

Wrap elements.

```
WrapMap([a, b, c, ...]) = [[a, b, c, ...]]
```

```rust
let wrap = WrapMap(&parser);
```

### Unwrap Map

Unwrap element.

```
WrapMap([a]) = a
```

```rust
let unwrap = UnwrapMap(&parser);
```

### Trim

Triming "a" by "-" yields: "--a--" -> "a"

```rust
let trim = Trim(&parser, &by);

// eg
let trim = Trim(&Token("a"), &Token("-"));
```
