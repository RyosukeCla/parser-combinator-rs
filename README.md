# Parser Combinator - rust implementation

Under construction..

## Usage

```rust
use parser::{
    parse, Char, Choice, ExtractMap, FlattenMap, Kind, Lazy, Many, RegExp, Seq, Token, WrapMap,
};

#[derive(Clone, Debug)]
enum Symbol {
    Num,
    Op,
    Expr,
}

// Expression Parser
pub fn main() {
    let num = Kind(&RegExp(r"([1-9][0-9]*|[0-9])"), Symbol::Num); // grant Num label
    let operator = Kind(&Char("+-"), Symbol::Op); // grand Op label
    let parenthesis = Lazy(); // lazy initialized parser
    let atom = Choice(&num).or(&parenthesis);
    let expression =
        FlattenMap(&Seq(&WrapMap(&atom)).and(&FlattenMap(&Many(&Seq(&operator).and(&atom)))));

    parenthesis.set_parser(&FlattenMap(&ExtractMap(
        &Seq(&Token("(")).and(&expression).and(&Token(")")),
        1, // extract expression
    )));

    let parser = Kind(&expression, Symbol::Expr); // grant Expr label

    let targets = vec!["1+2-(3+1-(4))", "hoge", "1+2-(3+1", "0-3+(((3)))"];

    for target in targets {
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
}

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
pub struct State<K: Clone> {
  pub success: bool,
  pub node: Option<Node<K>>,
  pub position: usize,
}

#[derive(Debug, Clone)]
pub struct Node<K: Clone> {
  pub value: Option<String>,
  pub children: Option<Vec<Node<K>>>,
  pub kind: Option<K>,
}

pub trait Parser<K: Clone> {
  fn parse(&self, target: &str, position: usize) -> State<K>;
  fn box_clone(&self) -> Box<Parser<K>>;
}
```

## Combinators

### Kind

Grant label

```rust
#[derive(Clone, Debug)]
enum Symbol {
  Num
}

let num = Kind(&RegExp(r"([1-9][0-9]*|[0-9])"), Symbol::Num);
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

### Extract Map

Extract an element from elements.

```
ExtractMap([ a, b, c ], 1) = [ b ]
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
