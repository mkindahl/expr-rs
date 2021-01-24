# expr-rs: recursive decent expression parser

An implementation of an simple expression parser with a focus on being
fast, compact, simple to embed, and with a minimum of
dependencies. The parser reads a string to build an expression
tree. The tree can then be evaluated, inspected, or manipulated.

The grammar of expressions is straightforward and (currently) quite
basic:

    expr   ::= `term` (("+" | "-") `term`)*
    term   ::= `factor` (("*" | "/") `factor`)*
    factor ::= ("+" | "-")? (`number` | `variable` | "(" `expr` ")")

The parser is implemented as a library to ensure that it can easily be
embedded into other applications and only has dependencies on the
standard library to be as lightweight as possible. A typical usage
could be:

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



