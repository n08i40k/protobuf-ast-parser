//! AST definitions for parsed Protocol Buffers files.
//!
//! # Examples
//! ```rust
//! use protobuf_ast_parser::ast::{Field, FieldModifier, Message, MessageEntry, RootEntry};
//!
//! let field = Field::new(Some(FieldModifier::Optional), "string", "name", 1, vec![]);
//! let message = Message::new("User", vec![MessageEntry::Field(field)]);
//! let file = vec![RootEntry::message(message)];
//! assert_eq!(file.len(), 1);
//! ```

use ownable::traits::IntoOwned;
use ownable::IntoOwned;
use std::borrow::Cow;
use std::collections::HashMap;

/// Represents a reserved or extensions range in `.proto` syntax.
///
/// # Examples
/// ```rust
/// use protobuf_ast_parser::ast::Range;
///
/// let finite = Range::from(1..5);
/// let open_ended = Range::from(10..);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Range {
    Default(std::ops::Range<i64>),
    From(std::ops::RangeFrom<i64>),
}

impl IntoOwned for Range {
    type Owned = Self;

    fn into_owned(self) -> Self::Owned {
        self
    }
}

impl From<std::ops::Range<i64>> for Range {
    fn from(range: std::ops::Range<i64>) -> Self {
        Self::Default(range)
    }
}

impl From<std::ops::RangeFrom<i64>> for Range {
    fn from(range: std::ops::RangeFrom<i64>) -> Self {
        Self::From(range)
    }
}

/// Option values and literal constants that can appear in `.proto` files.
///
/// # Examples
/// ```rust
/// use protobuf_ast_parser::ast::{Map, MapValue};
/// use std::borrow::Cow;
///
/// let map: Map = [(Cow::from("enabled"), MapValue::boolean(true))].into();
/// let value = MapValue::map(map);
/// ```
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum MapValue<'a> {
    Boolean(bool),
    Integer(i64),
    Ident(Cow<'a, str>),
    String(Cow<'a, str>),
    Map(Map<'a>),
}

impl<'a> MapValue<'a> {
    pub fn boolean(boolean: bool) -> Self {
        Self::Boolean(boolean)
    }

    pub fn integer(integer: i64) -> Self {
        Self::Integer(integer)
    }

    pub fn ident(ident: &'a str) -> Self {
        Self::Ident(Cow::from(ident))
    }

    pub fn string(string: &'a str) -> Self {
        Self::String(Cow::from(string))
    }

    pub fn map(map: Map<'a>) -> Self {
        Self::Map(map)
    }
}

/// Map literal used by options and aggregate constants.
pub type Map<'a> = HashMap<Cow<'a, str>, MapValue<'a>>;

/// Helper for building a [`Map`] from borrowed keys.
pub trait FromBorrowedIter<'a> {
    type Item;

    fn from_borrowed_iter<T: IntoIterator<Item = Self::Item>>(iter: T) -> Self;
}

impl<'a> FromBorrowedIter<'a> for Map<'a> {
    type Item = (&'a str, MapValue<'a>);

    fn from_borrowed_iter<T: IntoIterator<Item = (&'a str, MapValue<'a>)>>(iter: T) -> Self {
        let iter = iter.into_iter().map(|(key, value)| (Cow::from(key), value));
        Self::from_iter(iter)
    }
}

/// Represents an `option` statement or an inline option list entry.
///
/// # Examples
/// ```rust
/// use protobuf_ast_parser::ast::{MapValue, Option};
///
/// let option = Option::new("deprecated", MapValue::boolean(true));
/// assert_eq!(option.key, "deprecated");
/// ```
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct Option<'a> {
    pub key: Cow<'a, str>,
    pub value: MapValue<'a>,
}

impl<'a> Option<'a> {
    pub fn new(key: &'a str, value: MapValue<'a>) -> Self {
        Self {
            key: Cow::from(key),
            value,
        }
    }
}

/// A parsed comment with both raw source and trimmed text.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct Comment<'a> {
    pub r#type: CommentType,
    pub source: Cow<'a, str>,
    pub text: Cow<'a, str>,
}

impl<'a> Comment<'a> {
    pub fn new(r#type: CommentType, source: &'a str, text: &'a str) -> Self {
        Self {
            r#type,
            text: Cow::from(text),
            source: Cow::from(source),
        }
    }

    pub fn single_line(source: &'a str) -> Self {
        Self {
            r#type: CommentType::SingleLine,
            text: Cow::from(source[2..].trim()),
            source: Cow::from(source),
        }
    }

    pub fn multi_line(source: &'a str) -> Self {
        Self {
            r#type: CommentType::MultiLine,
            text: Cow::from(source[2..source.len() - 2].trim()),
            source: Cow::from(source),
        }
    }
}

/// Comment type markers for single-line (`//`) and multi-line (`/* */`) comments.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum CommentType {
    SingleLine,
    MultiLine,
}

/// Top-level entries in a `.proto` file.
///
/// # Examples
/// ```rust
/// use protobuf_ast_parser::ast::{RootEntry, Comment};
///
/// let entry = RootEntry::comment(Comment::single_line("// hi"));
/// ```
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum RootEntry<'a> {
    Comment(Comment<'a>),
    Syntax(Cow<'a, str>),
    Package(Cow<'a, str>),
    Import(Cow<'a, str>),
    Option(Option<'a>),
    Service(Service<'a>),
    Message(Message<'a>),
    Extend(Extend<'a>),
    Enum(Enum<'a>),
}

impl<'a> RootEntry<'a> {
    pub fn syntax(value: &'a str) -> Self {
        Self::Syntax(Cow::from(value))
    }

    pub fn comment(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }

    pub fn package(value: &'a str) -> Self {
        Self::Package(Cow::from(value))
    }

    pub fn import(value: &'a str) -> Self {
        Self::Import(Cow::from(value))
    }

    pub fn option(option: Option<'a>) -> Self {
        Self::Option(option)
    }

    pub fn service(service: Service<'a>) -> Self {
        Self::Service(service)
    }

    pub fn message(message: Message<'a>) -> Self {
        Self::Message(message)
    }

    pub fn extend(extend: Extend<'a>) -> Self {
        Self::Extend(extend)
    }

    pub fn r#enum(r#enum: Enum<'a>) -> Self {
        Self::Enum(r#enum)
    }
}

/// Alias for a full `.proto` file AST.
pub type Root<'a> = Vec<RootEntry<'a>>;

/// Service definition with its RPC entries.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct Service<'a> {
    pub ident: Cow<'a, str>,
    pub entries: Vec<ServiceEntry<'a>>,
}

impl<'a> Service<'a> {
    pub fn new(ident: &'a str, entries: Vec<ServiceEntry<'a>>) -> Self {
        Self {
            ident: Cow::from(ident),
            entries,
        }
    }
}

/// Entries that can appear inside a `service` block.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum ServiceEntry<'a> {
    Comment(Comment<'a>),
    Option(Option<'a>),

    Rpc(Rpc<'a>),
}

impl<'a> ServiceEntry<'a> {
    pub fn comment(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }

    pub fn option(option: Option<'a>) -> Self {
        Self::Option(option)
    }

    pub fn rpc(rpc: Rpc<'a>) -> Self {
        Self::Rpc(rpc)
    }
}

/// RPC definition inside a `service`.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct Rpc<'a> {
    pub ident: Cow<'a, str>,

    pub request: Cow<'a, str>,
    pub reply: Cow<'a, str>,

    pub stream: RpcStream,
}

impl<'a> Rpc<'a> {
    pub fn new(ident: &'a str, request: &'a str, reply: &'a str, stream: RpcStream) -> Self {
        Self {
            ident: Cow::from(ident),
            request: Cow::from(request),
            reply: Cow::from(reply),
            stream,
        }
    }
}

/// Streaming mode for an RPC definition.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum RpcStream {
    None,
    ClientBound,
    ServerBound,
    Bidirectional,
}

impl RpcStream {
    pub fn new(server_bound: bool, client_bound: bool) -> Self {
        match (server_bound, client_bound) {
            (true, true) => Self::Bidirectional,
            (true, false) => Self::ServerBound,
            (false, true) => Self::ClientBound,
            _ => Self::None,
        }
    }
}

/// Message definition with nested entries.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct Message<'a> {
    pub ident: Cow<'a, str>,
    pub entries: Vec<MessageEntry<'a>>,
}

impl<'a> Message<'a> {
    pub fn new(ident: &'a str, entries: Vec<MessageEntry<'a>>) -> Self {
        Self {
            ident: Cow::from(ident),
            entries,
        }
    }

    pub fn empty(ident: &'a str) -> Self {
        Self {
            ident: Cow::from(ident),
            entries: vec![],
        }
    }
}

/// Entries that can appear inside a `message` block.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum MessageEntry<'a> {
    Comment(Comment<'a>),
    Option(Option<'a>),

    Field(Field<'a>),
    OneOf(OneOf<'a>),
    Message(Message<'a>),
    Extend(Extend<'a>),
    Enum(Enum<'a>),

    ReservedIndices(Vec<Range>),
    ReservedIdents(Vec<Cow<'a, str>>),

    Extensions(Vec<Range>),
}

impl<'a> MessageEntry<'a> {
    pub fn comment(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }

    pub fn option(option: Option<'a>) -> Self {
        Self::Option(option)
    }

    pub fn field(field: Field<'a>) -> Self {
        Self::Field(field)
    }

    pub fn one_of(one_of: OneOf<'a>) -> Self {
        Self::OneOf(one_of)
    }

    pub fn message(message: Message<'a>) -> Self {
        Self::Message(message)
    }

    pub fn extend(extend: Extend<'a>) -> Self {
        Self::Extend(extend)
    }

    pub fn r#enum(r#enum: Enum<'a>) -> Self {
        Self::Enum(r#enum)
    }

    pub fn reserved_indices(ranges: Vec<Range>) -> Self {
        Self::ReservedIndices(ranges)
    }

    pub fn reserved_idents(idents: impl IntoIterator<Item = &'a str>) -> Self {
        Self::ReservedIdents(idents.into_iter().map(Cow::from).collect())
    }

    pub fn extensions(ranges: Vec<Range>) -> Self {
        Self::Extensions(ranges)
    }
}

/// Field definition inside a message, oneof, or extend block.
///
/// # Examples
/// ```rust
/// use protobuf_ast_parser::ast::{Field, FieldModifier};
///
/// let field = Field::new(Some(FieldModifier::Optional), "string", "name", 1, vec![]);
/// assert_eq!(field.index, 1);
/// ```
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct Field<'a> {
    pub modifier: std::option::Option<FieldModifier>,
    pub r#type: Cow<'a, str>,
    pub ident: Cow<'a, str>,
    pub index: i64,
    pub options: Vec<Option<'a>>,
}

impl<'a> Field<'a> {
    pub fn new(
        modifier: std::option::Option<FieldModifier>,
        r#type: &'a str,
        ident: &'a str,
        index: i64,
        options: Vec<Option<'a>>,
    ) -> Self {
        Self {
            modifier,
            r#type: Cow::from(r#type),
            ident: Cow::from(ident),
            index,
            options,
        }
    }

    pub fn basic(r#type: &'a str, ident: &'a str, index: i64) -> Self {
        Self {
            modifier: None,
            r#type: Cow::from(r#type),
            ident: Cow::from(ident),
            index,
            options: vec![],
        }
    }
}

/// `oneof` definition inside a message.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct OneOf<'a> {
    pub ident: Cow<'a, str>,
    pub entries: Vec<OneOfEntry<'a>>,
}

impl<'a> OneOf<'a> {
    pub fn new(ident: &'a str, entries: Vec<OneOfEntry<'a>>) -> Self {
        Self {
            ident: Cow::from(ident),
            entries,
        }
    }
}

/// Entries that can appear inside a `oneof` block.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum OneOfEntry<'a> {
    Comment(Comment<'a>),
    Option(Option<'a>),

    Field(Field<'a>),
}

impl<'a> OneOfEntry<'a> {
    pub fn comment(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }

    pub fn option(option: Option<'a>) -> Self {
        Self::Option(option)
    }

    pub fn field(field: Field<'a>) -> Self {
        Self::Field(field)
    }
}

/// Field modifier keywords.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum FieldModifier {
    Optional,
    Required,
    Repeated,
}

/// Extend block definition.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct Extend<'a> {
    pub r#type: Cow<'a, str>,
    pub entries: Vec<ExtendEntry<'a>>,
}

impl<'a> Extend<'a> {
    pub fn new(r#type: &'a str, entries: Vec<ExtendEntry<'a>>) -> Self {
        Self {
            r#type: Cow::from(r#type),
            entries,
        }
    }
}

/// Entries that can appear inside an `extend` block.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum ExtendEntry<'a> {
    Comment(Comment<'a>),
    Field(Field<'a>),
}

impl<'a> ExtendEntry<'a> {
    pub fn comment(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }

    pub fn field(field: Field<'a>) -> Self {
        Self::Field(field)
    }
}

/// Enum definition.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct Enum<'a> {
    pub ident: Cow<'a, str>,
    pub entries: Vec<EnumEntry<'a>>,
}

impl<'a> Enum<'a> {
    pub fn new(ident: &'a str, entries: Vec<EnumEntry<'a>>) -> Self {
        Self {
            ident: Cow::from(ident),
            entries,
        }
    }
}

/// Entries that can appear inside an `enum` block.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum EnumEntry<'a> {
    Comment(Comment<'a>),
    Option(Option<'a>),
    Variant(EnumVariant<'a>),
}

impl<'a> EnumEntry<'a> {
    pub fn comment(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }

    pub fn option(option: Option<'a>) -> Self {
        Self::Option(option)
    }

    pub fn variant(ident: &'a str, value: i64, options: Vec<Option<'a>>) -> Self {
        Self::Variant(EnumVariant {
            ident: Cow::from(ident),
            value,
            options,
        })
    }
}

#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct EnumVariant<'a> {
    ident: Cow<'a, str>,
    value: i64,
    options: Vec<Option<'a>>,
}
