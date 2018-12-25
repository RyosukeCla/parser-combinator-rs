# Parser Combinator - rust implementation

## Combinators

### Token

```rs
let token = Token("token");
println!("{}", parse(&token, "token").unwrap());
// token
```

### Char

```rs
let operator = Char("+-");
println!("{}", parse(&operator, "+").unwrap());
// +
println!("{}", parse(&operator, "-").unwrap());
// -
```

### Regex

```rs
let number = RegExp(r"([0-9]|[1-9][0-9]*)");
println!("{}", parse(&number, "12345").unwrap());
// 12345
```

### Sequence

```rs
let seq = Seq(&Token("a")).and(&Token("b"));
println!("{}", parse(&seq, "ab").unwrap());
// [ a, b ]
```

### Many

```rs
let many = Many(&Token("a"));
println!("{}", parse(&many, "aaaaa").unwrap());
// [a, a, a, a, a]
```

### Option

```js
/(option)?/;
```

```rs
let option = Opt(&Token("aaa"));
println!("{}", parse(&option, "aaa").unwrap());
// aaa
println!("{}", parse(&option, "").unwrap());
//
```

### Choice

```rs
let choice = Choice(&Token("a")).or(&Token("b"));
println!("{}", parse(&choice, "a").unwrap());
// a
println!("{}", parse(&choice, "b").unwrap());
// b
```

### Lazy

Lazy initialized parser.
It is useful for making recursive parser.

```rs
let lazy = Lazy();
// define parsers
lazy.set_parser(&parser);
```

### Map

map node to new node.

```rs
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

```rs
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

```rs
let flatten = FlattenMap(&parser);
```

### Wrap Map

Wrap elements.

```
WrapMap([a, b, c, ...]) = [[a, b, c, ...]]
```

```rs
let wrap = WrapMap(&parser);
```
