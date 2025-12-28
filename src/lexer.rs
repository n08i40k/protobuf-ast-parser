//! Tokenization for Protocol Buffers sources.
//!
//! # Examples
//! ```rust
//! use protobuf_ast_parser::lexer::{Lexer, Token};
//!
//! let mut lexer = Lexer::new("syntax = \"proto3\";");
//! let first = lexer.next().unwrap().unwrap();
//! assert_eq!(first.1, Token::Syntax);
//! ```

use logos::{Logos, Span};
use std::num::{IntErrorKind, ParseIntError};

/// Categories of lexical errors produced by [`Lexer`].
#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalErrorKind {
    #[default]
    InvalidToken,
    InvalidInteger(ParseIntError),
}

impl From<ParseIntError> for LexicalErrorKind {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidInteger(value)
    }
}

/// Error emitted when the lexer cannot produce a valid token.
#[derive(Debug, Clone, PartialEq)]
pub struct LexicalError<'a> {
    kind: LexicalErrorKind,
    input: &'a str,
    span: Span,
}

impl<'a> std::fmt::Display for LexicalError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let line = self.input[..self.span.start]
            .chars()
            .filter(|&ch| ch == '\n')
            .count()
            + 1;

        let column = self.span.start - self.input[..self.span.start].rfind("\n").unwrap_or(0);

        let position = format!("line {}, column {}", line, column);

        match &self.kind {
            LexicalErrorKind::InvalidToken => write!(
                f,
                "Invalid token \"{}\" at {}",
                &self.input[self.span.start..self.span.end],
                position
            )?,
            LexicalErrorKind::InvalidInteger(inner) => write!(
                f,
                "Invalid number {} at {}: {}",
                &self.input[self.span.start..self.span.end],
                position,
                match inner.kind() {
                    IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => "overflow",
                    _ => "unknown",
                }
            )?,
        };

        Ok(())
    }
}

fn string_from_lexer<'a>(lex: &mut logos::Lexer<'a, Token<'a>>) -> &'a str {
    let slice = lex.slice();
    &slice[1..slice.len() - 1]
}

/// Token kinds produced by the lexer.
#[derive(Clone, Debug, PartialEq, Logos)]
#[logos(error = LexicalErrorKind)]
#[logos(skip r"[\s\t\n\f]+")]
pub enum Token<'a> {
    #[regex(r"//.*", allow_greedy = true)]
    SingleLineComment(&'a str),

    #[regex(r"\/\*[^*]*\*+(?:[^\/*][^*]*\*+)*\/")]
    MultiLineComment(&'a str),

    #[token("=")]
    Eq,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token(",")]
    Comma,

    #[token(".")]
    Period,

    #[token("(")]
    OpenPth,

    #[token(")")]
    ClosePth,

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token("<")]
    OpenAngle,

    #[token(">")]
    CloseAngle,

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Boolean(bool),

    #[regex(r"-?[0-9]+", |lex| lex.slice().parse())]
    #[regex(r"0x[0-9a-fA-F]{1,16}", |lex| i64::from_str_radix(&lex.slice()[2..], 16))]
    Integer(i64),

    #[token("to")]
    To,

    #[token("max")]
    Max,

    #[token("syntax")]
    Syntax,

    #[token("option")]
    Option,

    #[token("package")]
    Package,

    #[token("import")]
    Import,

    #[token("service")]
    Service,

    #[token("rpc")]
    Rpc,

    #[token("stream")]
    Stream,

    #[token("returns")]
    Returns,

    #[token("message")]
    Message,

    #[token("oneof")]
    OneOf,

    #[token("extend")]
    Extend,

    #[token("enum")]
    Enum,

    #[token("reserved")]
    Reserved,

    #[token("extensions")]
    Extensions,

    #[token("optional")]
    Optional,

    #[token("required")]
    Required,

    #[token("repeated")]
    Repeated,

    #[token("map")]
    Map,

    #[regex(r#"'((?:[^'\n]|(?:\\\'))*)'"#, string_from_lexer)]
    #[regex(r#""((?:[^"\n]|(?:\\\"))*)""#, string_from_lexer)]
    String(&'a str),

    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]*", priority = 0)]
    Ident(&'a str),
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Streaming lexer that yields spanned tokens.
pub struct Lexer<'input> {
    inner: logos::SpannedIter<'input, Token<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(src: &'input str) -> Self {
        Self {
            inner: Token::lexer(src).spanned(),
        }
    }
}

/// LALRPOP-compatible spanned token wrapper.
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, usize, LexicalError<'input>>;

    fn next(&mut self) -> Option<Self::Item> {
        let (tok, span) = self.inner.next()?;

        Some(
            tok.map(|tok| (span.start, tok, span.end))
                .map_err(|kind| LexicalError {
                    kind,
                    input: self.inner.source(),
                    span,
                }),
        )
    }
}
