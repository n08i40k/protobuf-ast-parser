use crate::{ast, lexer, proto};

/// Parse error type returned by [`parse`].
pub type ParseError<'a> =
    lalrpop_util::ParseError<usize, lexer::Token<'a>, lexer::LexicalError<'a>>;

/// Result alias for parsing `.proto` sources.
pub type ParseResult<'a> = Result<ast::Root<'a>, ParseError<'a>>;

/// Parse a Protocol Buffers source string into an AST.
///
/// # Examples
/// ```rust
/// use protobuf_ast_parser::parse;
///
/// let source = r#"syntax = "proto3"; message User { string name = 1; }"#;
/// let ast = parse(source).expect("valid proto");
/// assert!(!ast.is_empty());
/// ```
#[allow(clippy::needless_lifetimes)]
pub fn parse<'a>(data: &'a str) -> ParseResult<'a> {
    let lexer = lexer::Lexer::new(data);
    let parser = proto::RootParser::new();

    parser.parse(data, lexer)
}
