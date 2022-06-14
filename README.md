# expr-rs: recursive decent expression parser

An implementation of an simple expression parser with a focus on being
fast, compact, simple to embed, and with a minimum of
dependencies. The parser reads a string to build an expression
tree. The tree can then be evaluated, inspected, or manipulated.

## Grammar

The grammar of expressions is straightforward and (currently) quite
basic:

```ebnf
Expr = Term, { ("+" | "-"), Term }
Term = Factor, { ("*" | "/"), Factor}
Factor = [ "+" | "-" ], ( Number | Variable | "(", Expr, ")" )
```

## Example

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

## Docker image build

The repository includes a `Dockerfile` to build a docker image that
runs the `expr` executable. It is mostly intended to be an example of
how to use a multi-stage build to build and deploy a Rust application
as a Docker image without including anything else.

You can build an image `expr-rs:latest` using:

```
docker build -t expr-rs:latest .
```

This will build a Docker image based on the `scratch` image with a
statically linked Rust executable (using then musl library) that you
can run with arguments using:

```
docker run expr-rs:latest '2+5*3'
```
