use ownable::IntoOwned;
use std::borrow::Cow;
use std::collections::HashMap;

pub type File<'a> = Vec<FileEntry<'a>>;

#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum RangeEnd {
    Integer(i64),
    Max,
}

#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct Range {
    pub start: i64,
    pub end: RangeEnd,
}

impl From<i64> for Range {
    fn from(start: i64) -> Self {
        Self {
            start,
            end: RangeEnd::Integer(start + 1),
        }
    }
}

impl From<std::ops::Range<i64>> for Range {
    fn from(range: std::ops::Range<i64>) -> Self {
        Self {
            start: range.start,
            end: RangeEnd::Integer(range.end + 1),
        }
    }
}

impl From<(i64, ())> for Range {
    fn from(range: (i64, ())) -> Self {
        Self {
            start: range.0,
            end: RangeEnd::Max,
        }
    }
}

#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum MapValue<'a> {
    Boolean(bool),
    Integer(i64),
    Ident(Cow<'a, str>),
    String(Cow<'a, str>),
    Map(JSONLikeMap<'a>),
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

    pub fn map(map: JSONLikeMap<'a>) -> Self {
        Self::Map(map)
    }
}

pub type JSONLikeMap<'a> = HashMap<Cow<'a, str>, MapValue<'a>>;

pub trait JSONLikeMapTrait<'a> {
    fn from_borrowed_iter<T: IntoIterator<Item = (&'a str, MapValue<'a>)>>(iter: T) -> Self;
}

impl<'a> JSONLikeMapTrait<'a> for JSONLikeMap<'a> {
    fn from_borrowed_iter<T: IntoIterator<Item = (&'a str, MapValue<'a>)>>(iter: T) -> Self {
        let iter = iter.into_iter().map(|(key, value)| (Cow::from(key), value));
        Self::from_iter(iter)
    }
}

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

#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum CommentType {
    SingleLine,
    MultiLine,
}

#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum FileEntry<'a> {
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

impl<'a> FileEntry<'a> {
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

#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct Field<'a> {
    pub modifier: FieldModifier,
    pub r#type: Cow<'a, str>,
    pub ident: Cow<'a, str>,
    pub index: i64,
    pub options: Vec<Option<'a>>,
}

impl<'a> Field<'a> {
    pub fn new(
        modifier: FieldModifier,
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
            modifier: FieldModifier::None,
            r#type: Cow::from(r#type),
            ident: Cow::from(ident),
            index,
            options: vec![],
        }
    }
}

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

#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum FieldModifier {
    None,
    Optional,
    Required,
    Repeated,
}

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

#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum EnumEntry<'a> {
    Comment(Comment<'a>),
    Option(Option<'a>),
    Pair {
        ident: Cow<'a, str>,
        value: i64,
        options: Vec<Option<'a>>,
    },
}

impl<'a> EnumEntry<'a> {
    pub fn comment(comment: Comment<'a>) -> Self {
        Self::Comment(comment)
    }

    pub fn option(option: Option<'a>) -> Self {
        Self::Option(option)
    }

    pub fn pair(ident: &'a str, value: i64, options: Vec<Option<'a>>) -> Self {
        Self::Pair {
            ident: Cow::from(ident),
            value,
            options,
        }
    }
}
