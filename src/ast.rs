//! AST definitions for parsed Protocol Buffers files.
//!
//! # Examples
//! ```rust
//! use protobuf_ast_parser::ast::{Field, FieldModifier, Message, MessageEntry, RootEntry};
//!
//! let field = Field::new(Some(FieldModifier::Optional), "string", "name", 1, vec![]);
//! let message = Message::new("User", vec![MessageEntry::Field(field)]);
//! let file = vec![RootEntry::from(message)];
//! assert_eq!(file.len(), 1);
//! ```

use ownable::traits::IntoOwned;
use ownable::IntoOwned;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

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
/// let map: Map = [(Cow::from("enabled"), MapValue::from(true))].into();
/// let value = MapValue::from(map);
/// ```
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum MapValue<'a> {
    Boolean(bool),
    Integer(i64),
    Ident(Cow<'a, str>),
    String(Cow<'a, str>),
    Map(Map<'a>),
}

impl<'a> From<bool> for MapValue<'a> {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl<'a> From<i64> for MapValue<'a> {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl<'a> From<Map<'a>> for MapValue<'a> {
    fn from(value: Map<'a>) -> Self {
        Self::Map(value)
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
/// let option = Option::new("deprecated", MapValue::from(true));
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
/// let entry = RootEntry::from(Comment::single_line("// hi"));
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

impl<'a> From<Comment<'a>> for RootEntry<'a> {
    fn from(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }
}

impl<'a> From<Option<'a>> for RootEntry<'a> {
    fn from(option: Option<'a>) -> Self {
        Self::Option(option)
    }
}

impl<'a> From<Service<'a>> for RootEntry<'a> {
    fn from(service: Service<'a>) -> Self {
        Self::Service(service)
    }
}

impl<'a> From<Message<'a>> for RootEntry<'a> {
    fn from(message: Message<'a>) -> Self {
        Self::Message(message)
    }
}

impl<'a> From<Extend<'a>> for RootEntry<'a> {
    fn from(extend: Extend<'a>) -> Self {
        Self::Extend(extend)
    }
}

impl<'a> From<Enum<'a>> for RootEntry<'a> {
    fn from(r#enum: Enum<'a>) -> Self {
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

impl<'a> From<Comment<'a>> for ServiceEntry<'a> {
    fn from(comment: Comment<'a>) -> Self {
        ServiceEntry::Comment(comment)
    }
}

impl<'a> From<Option<'a>> for ServiceEntry<'a> {
    fn from(option: Option<'a>) -> Self {
        ServiceEntry::Option(option)
    }
}

impl<'a> From<Rpc<'a>> for ServiceEntry<'a> {
    fn from(rpc: Rpc<'a>) -> Self {
        ServiceEntry::Rpc(rpc)
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

#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct ReservedIndices(Vec<Range>);

impl From<Vec<Range>> for ReservedIndices {
    fn from(value: Vec<Range>) -> Self {
        ReservedIndices(value)
    }
}

impl Into<Vec<Range>> for ReservedIndices {
    fn into(self) -> Vec<Range> {
        self.0
    }
}

impl Deref for ReservedIndices {
    type Target = Vec<Range>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ReservedIndices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct ReservedIdents<'a>(Vec<Cow<'a, str>>);

impl<'a> From<Vec<&'a str>> for ReservedIdents<'a> {
    fn from(value: Vec<&'a str>) -> Self {
        ReservedIdents(value.iter().map(|s| Cow::from(*s)).collect())
    }
}

impl<'a> From<Vec<Cow<'a, str>>> for ReservedIdents<'a> {
    fn from(value: Vec<Cow<'a, str>>) -> Self {
        ReservedIdents(value)
    }
}

impl<'a> Into<Vec<Cow<'a, str>>> for ReservedIdents<'a> {
    fn into(self) -> Vec<Cow<'a, str>> {
        self.0
    }
}

impl<'a> Deref for ReservedIdents<'a> {
    type Target = Vec<Cow<'a, str>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for ReservedIdents<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct Extensions(Vec<Range>);

impl From<Vec<Range>> for Extensions {
    fn from(value: Vec<Range>) -> Self {
        Extensions(value)
    }
}

impl Into<Vec<Range>> for Extensions {
    fn into(self) -> Vec<Range> {
        self.0
    }
}

impl Deref for Extensions {
    type Target = Vec<Range>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

    ReservedIndices(ReservedIndices),
    ReservedIdents(ReservedIdents<'a>),

    Extensions(Extensions),
}

impl<'a> From<Comment<'a>> for MessageEntry<'a> {
    fn from(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }
}

impl<'a> From<Option<'a>> for MessageEntry<'a> {
    fn from(option: Option<'a>) -> Self {
        Self::Option(option)
    }
}

impl<'a> From<Field<'a>> for MessageEntry<'a> {
    fn from(field: Field<'a>) -> Self {
        Self::Field(field)
    }
}

impl<'a> From<OneOf<'a>> for MessageEntry<'a> {
    fn from(one_of: OneOf<'a>) -> Self {
        Self::OneOf(one_of)
    }
}

impl<'a> From<Message<'a>> for MessageEntry<'a> {
    fn from(message: Message<'a>) -> Self {
        Self::Message(message)
    }
}

impl<'a> From<Extend<'a>> for MessageEntry<'a> {
    fn from(extend: Extend<'a>) -> Self {
        Self::Extend(extend)
    }
}

impl<'a> From<Enum<'a>> for MessageEntry<'a> {
    fn from(r#enum: Enum<'a>) -> Self {
        Self::Enum(r#enum)
    }
}

impl<'a> From<ReservedIndices> for MessageEntry<'a> {
    fn from(reserved_indices: ReservedIndices) -> Self {
        Self::ReservedIndices(reserved_indices)
    }
}

impl<'a> From<ReservedIdents<'a>> for MessageEntry<'a> {
    fn from(reserved_idents: ReservedIdents<'a>) -> Self {
        Self::ReservedIdents(reserved_idents)
    }
}

impl<'a> From<Extensions> for MessageEntry<'a> {
    fn from(extensions: Extensions) -> Self {
        Self::Extensions(extensions)
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

impl<'a> From<Comment<'a>> for OneOfEntry<'a> {
    fn from(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }
}

impl<'a> From<Option<'a>> for OneOfEntry<'a> {
    fn from(option: Option<'a>) -> Self {
        Self::Option(option)
    }
}

impl<'a> From<Field<'a>> for OneOfEntry<'a> {
    fn from(field: Field<'a>) -> Self {
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

impl<'a> From<Comment<'a>> for ExtendEntry<'a> {
    fn from(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }
}

impl<'a> From<Field<'a>> for ExtendEntry<'a> {
    fn from(field: Field<'a>) -> Self {
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

impl<'a> From<Comment<'a>> for EnumEntry<'a> {
    fn from(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }
}

impl<'a> From<Option<'a>> for EnumEntry<'a> {
    fn from(option: Option<'a>) -> Self {
        Self::Option(option)
    }
}

impl<'a> From<EnumVariant<'a>> for EnumEntry<'a> {
    fn from(value: EnumVariant<'a>) -> Self {
        Self::Variant(value)
    }
}

/// Enum variant definition inside an enum block.
///
/// # Examples
/// ```rust
/// use protobuf_ast_parser::ast::EnumVariant;
///
/// let variant = EnumVariant::new("FIRST", 1, vec![]);
/// assert_eq!(variant.value, 1);
/// ```
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct EnumVariant<'a> {
    pub ident: Cow<'a, str>,
    pub value: i64,
    pub options: Vec<Option<'a>>,
}

impl<'a> EnumVariant<'a> {
    pub fn new(ident: &'a str, value: i64, options: Vec<Option<'a>>) -> Self {
        Self {
            ident: Cow::from(ident),
            value,
            options,
        }
    }
}
