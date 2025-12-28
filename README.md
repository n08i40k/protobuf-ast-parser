# protobuf-parser

A Rust parser for Protocol Buffers (proto2 and proto3) built with LALRPOP + Logos.
It focuses on turning `.proto` files into a lightweight AST while preserving comments,
which is handy for tooling, linting, and analysis workflows.

## Usage

```rust
use protobuf_parser::parse;

let source = r#"
syntax = "proto3";
message User { string name = 1; }
"#;

let ast = parse(source)?;
```
