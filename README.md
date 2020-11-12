# Recursive decent expression parser implemented in Rust

This will allow parsing a string and evaluating it as an
expression. It is implemented as a library so that it can be embedded
into other applications and only has dependencies on the standard
library to be as lightweight as possible.

The grammar of expressions is straightforward:

    expr   ::= `term` (("+" | "-") `term`)*
    term   ::= `factor` (("*" | "/") `factor`)*
    factor ::= `number` | `variable` | "(" `expr` ")"

The parser is implemented as a predictive parser, handles variables,
and embedding the parser into a program is straightforward.

```rust
 use expr::parse;
 use std::collections::HashMap;

 fn main() {
   let tree = parse("2 * x + 19").unwrap();
   let mut vars = HashMap::new();
   map.insert("x".to_string(), 12.0);
   println!("{} = {}", tree, tree.eval(&map));
 }
```

A sample application that just evaluates the expression with optional
assignments are provided with the package.



