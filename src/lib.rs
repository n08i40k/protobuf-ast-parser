//! Protocol Buffers (proto2/proto3) parser that produces a typed AST.
//!
//! # Examples
//! ```rust
//! use protobuf_ast_parser::parse;
//!
//! let source = r#"syntax = "proto3"; message User { string name = 1; }"#;
//! let ast = parse(source).expect("valid proto");
//! assert!(!ast.is_empty());
//! ```

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub proto
);

pub mod ast;
pub mod lexer;
mod parser;

pub use ast::Root;
pub use parser::{parse, ParseError, ParseResult};

#[cfg(test)]
mod tests;
