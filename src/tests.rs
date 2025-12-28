use crate::{ast, parse};
use std::borrow::Cow;

macro_rules! parse_ast {
    ($file:literal) => {{
        let data = include_str!(concat!("../proto/tests/", $file));

        match parse(&data) {
            Err(error) => panic!("{}", error),
            Ok(ast) => ast,
        }
    }};
}

#[test]
fn empty() {
    let ast = parse_ast!("empty.proto");
    assert!(ast.is_empty());
}

#[test]
fn syntax() {
    let ast = parse_ast!("syntax.proto");
    let target_ast = vec![ast::RootEntry::Syntax(Cow::from("proto3"))];

    assert_eq!(ast, target_ast);
}

#[test]
fn package_simple() {
    let ast = parse_ast!("package-simple.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto3")),
        ast::RootEntry::Package(Cow::from("mypkg")),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn package_complex() {
    let ast = parse_ast!("package-complex.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto3")),
        ast::RootEntry::Package(Cow::from("my.pkg")),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn import() {
    let ast = parse_ast!("import.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto3")),
        ast::RootEntry::Import(Cow::from("google/protobuf/any.proto")),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn message_empty() {
    let ast = parse_ast!("message-empty.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto3")),
        ast::RootEntry::from(ast::Message::empty("Empty")),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn message() {
    let ast = parse_ast!("message.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto3")),
        ast::RootEntry::from(ast::Message::new(
            "Message",
            vec![
                ast::MessageEntry::from(ast::ReservedIndices::from(vec![
                    ast::Range::from(2..3),
                    ast::Range::from(6..),
                ])),
                ast::MessageEntry::ReservedIdents(ast::ReservedIdents::from(vec!["sample"])),
                ast::MessageEntry::from(ast::Field::new(None, "bool", "first", 1, vec![])),
                ast::MessageEntry::from(ast::Field::new(
                    Some(ast::FieldModifier::Optional),
                    "string",
                    "third",
                    3,
                    vec![],
                )),
                ast::MessageEntry::from(ast::Field::new(
                    Some(ast::FieldModifier::Repeated),
                    "uint64",
                    "fourth",
                    4,
                    vec![],
                )),
                ast::MessageEntry::from(ast::Field::new(
                    None,
                    "map<string, string>",
                    "fifth",
                    5,
                    vec![],
                )),
            ],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn message_inner() {
    let ast = parse_ast!("message-inner.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto3")),
        ast::RootEntry::from(ast::Message::new(
            "Parent",
            vec![
                ast::MessageEntry::from(ast::Message::new(
                    "Child",
                    vec![ast::MessageEntry::from(ast::Field::new(
                        None,
                        "bool",
                        "var",
                        1,
                        vec![],
                    ))],
                )),
                ast::MessageEntry::from(ast::Field::new(None, "Child", "child", 1, vec![])),
            ],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn r#enum() {
    let ast = parse_ast!("enum.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto3")),
        ast::RootEntry::from(ast::Enum::new(
            "Enum",
            vec![
                ast::EnumEntry::from(ast::EnumVariant::new("ZERO", 0, vec![])),
                ast::EnumEntry::from(ast::EnumVariant::new("POSITIVE", 1, vec![])),
                ast::EnumEntry::from(ast::EnumVariant::new("NEGATIVE", -1, vec![])),
            ],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn options() {
    let ast = parse_ast!("options.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto3")),
        ast::RootEntry::Import(Cow::from("google/protobuf/descriptor.proto")),
        ast::RootEntry::from(ast::Option::new(
            "java_multiple_files",
            ast::MapValue::from(true),
        )),
        ast::RootEntry::from(ast::Option::new(
            "java_package",
            ast::MapValue::String(Cow::from("xd.xd")),
        )),
        ast::RootEntry::from(ast::Extend::new(
            "google.protobuf.EnumValueOptions",
            vec![ast::ExtendEntry::from(ast::Field::new(
                Some(ast::FieldModifier::Optional),
                "bool",
                "own_enum_value",
                2000,
                vec![],
            ))],
        )),
        ast::RootEntry::from(ast::Extend::new(
            "google.protobuf.FieldOptions",
            vec![ast::ExtendEntry::from(ast::Field::new(
                Some(ast::FieldModifier::Optional),
                "bool",
                "own_field_value",
                2000,
                vec![ast::Option::new("deprecated", ast::MapValue::from(true))],
            ))],
        )),
        ast::RootEntry::from(ast::Enum::new(
            "Enum",
            vec![
                ast::EnumEntry::from(ast::Option::new("allow_alias", ast::MapValue::from(true))),
                ast::EnumEntry::Variant(ast::EnumVariant::new(
                    "FIRST",
                    0,
                    vec![ast::Option::new("deprecated", ast::MapValue::from(true))],
                )),
                ast::EnumEntry::from(ast::EnumVariant::new(
                    "SECOND",
                    0,
                    vec![ast::Option::new(
                        "(own_enum_value)",
                        ast::MapValue::from(true),
                    )],
                )),
            ],
        )),
        ast::RootEntry::from(ast::Message::new(
            "Message",
            vec![
                ast::MessageEntry::from(ast::Option::new("deprecated", ast::MapValue::from(true))),
                ast::MessageEntry::from(ast::Field::new(
                    Some(ast::FieldModifier::Optional),
                    "bool",
                    "var",
                    1,
                    vec![
                        ast::Option::new("deprecated", ast::MapValue::from(true)),
                        ast::Option::new("(own_field_value)", ast::MapValue::from(false)),
                        ast::Option::new(
                            "edition_defaults",
                            ast::MapValue::from(ast::Map::from([
                                (
                                    Cow::from("edition"),
                                    ast::MapValue::Ident(Cow::from("EDITION_PROTO2")),
                                ),
                                (Cow::from("value"), ast::MapValue::String(Cow::from("true"))),
                            ])),
                        ),
                        ast::Option::new(
                            "edition_defaults",
                            ast::MapValue::from(ast::Map::from([
                                (
                                    Cow::from("edition"),
                                    ast::MapValue::Ident(Cow::from("EDITION_PROTO3")),
                                ),
                                (
                                    Cow::from("value"),
                                    ast::MapValue::String(Cow::from("false")),
                                ),
                            ])),
                        ),
                    ],
                )),
            ],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn comments() {
    let ast = parse_ast!("comments.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto3")),
        ast::RootEntry::Import(Cow::from("google/protobuf/descriptor.proto")),
        ast::RootEntry::from(ast::Comment::single_line("// single line comment")),
        ast::RootEntry::from(ast::Comment::single_line("// another single line comment")),
        ast::RootEntry::from(ast::Comment::multi_line("/* multi\n   line\n   comment */")),
        ast::RootEntry::from(ast::Message::new(
            "Message",
            vec![
                ast::MessageEntry::from(ast::Comment::single_line("// in message")),
                ast::MessageEntry::from(ast::Field::new(None, "bool", "var", 1, vec![])),
                ast::MessageEntry::from(ast::Comment::single_line("// right after entry")),
                ast::MessageEntry::from(ast::Comment::single_line("// at the bottom")),
            ],
        )),
        ast::RootEntry::from(ast::Enum::new(
            "Enum",
            vec![
                ast::EnumEntry::from(ast::Comment::single_line("// in enum")),
                ast::EnumEntry::from(ast::EnumVariant::new("DEFAULT", 0, vec![])),
            ],
        )),
        ast::RootEntry::from(ast::Extend::new(
            "google.protobuf.FieldOptions",
            vec![
                ast::ExtendEntry::from(ast::Comment::single_line("// in extend")),
                ast::ExtendEntry::from(ast::Field::new(
                    Some(ast::FieldModifier::Optional),
                    "bool",
                    "var",
                    1,
                    vec![],
                )),
            ],
        )),
        ast::RootEntry::from(ast::Comment::single_line("// at the bottom of the file")),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn extensions() {
    let ast = parse_ast!("extensions.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto2")),
        ast::RootEntry::from(ast::Message::new(
            "Message",
            vec![ast::MessageEntry::from(ast::Extensions::from(vec![
                ast::Range::from(1..2),
                ast::Range::from(2..5),
                ast::Range::from(6..),
            ]))],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn required() {
    let ast = parse_ast!("required.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto2")),
        ast::RootEntry::from(ast::Message::new(
            "Message",
            vec![ast::MessageEntry::from(ast::Field::new(
                Some(ast::FieldModifier::Required),
                "bool",
                "var",
                1,
                vec![],
            ))],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn keywords() {
    let ast = parse_ast!("keywords.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto3")),
        ast::RootEntry::from(ast::Message::empty("Ident")),
        ast::RootEntry::from(ast::Message::new(
            "to",
            vec![ast::MessageEntry::from(ast::Message::empty("inner"))],
        )),
        ast::RootEntry::from(ast::Message::empty("max")),
        ast::RootEntry::from(ast::Message::empty("syntax")),
        ast::RootEntry::from(ast::Message::empty("option")),
        ast::RootEntry::from(ast::Message::empty("package")),
        ast::RootEntry::from(ast::Message::empty("import")),
        ast::RootEntry::from(ast::Message::empty("message")),
        ast::RootEntry::from(ast::Message::empty("oneof")),
        ast::RootEntry::from(ast::Message::empty("extend")),
        ast::RootEntry::from(ast::Message::empty("enum")),
        ast::RootEntry::from(ast::Message::empty("reserved")),
        ast::RootEntry::from(ast::Message::empty("extensions")),
        ast::RootEntry::from(ast::Message::empty("optional")),
        ast::RootEntry::from(ast::Message::empty("required")),
        ast::RootEntry::from(ast::Message::empty("repeated")),
        ast::RootEntry::from(ast::Message::empty("map")),
        ast::RootEntry::from(ast::Message::new(
            "Message",
            vec![
                ast::MessageEntry::from(ast::Field::new(None, "bool", "var1", 1, vec![])),
                ast::MessageEntry::from(ast::Field::new(None, "Ident", "var2", 2, vec![])),
                ast::MessageEntry::from(ast::Field::new(None, "to", "var3", 3, vec![])),
                ast::MessageEntry::from(ast::Field::new(None, "to.inner", "var4", 4, vec![])),
                ast::MessageEntry::from(ast::Field::new(None, "max", "var5", 5, vec![])),
                ast::MessageEntry::from(ast::Field::new(None, "syntax", "var6", 6, vec![])),
                ast::MessageEntry::from(ast::Field::new(None, "package", "var7", 7, vec![])),
                ast::MessageEntry::from(ast::Field::new(None, "import", "var8", 8, vec![])),
            ],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn oneof() {
    let ast = parse_ast!("oneof.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto3")),
        ast::RootEntry::from(ast::Message::new(
            "Message",
            vec![
                ast::MessageEntry::from(ast::OneOf::new(
                    "OneOf",
                    vec![
                        ast::OneOfEntry::from(ast::Option::new(
                            "uninterpreted_option",
                            ast::MapValue::from(ast::Map::from([(
                                Cow::from("string_value"),
                                ast::MapValue::String(Cow::from("")),
                            )])),
                        )),
                        ast::OneOfEntry::from(ast::Field::new(
                            None,
                            "bool",
                            "oneof_var",
                            1,
                            vec![],
                        )),
                    ],
                )),
                ast::MessageEntry::from(ast::Field::new(None, "bool", "message_var", 2, vec![])),
            ],
        )),
    ];

    assert_eq!(ast, target_ast);
}

#[test]
fn service() {
    let ast = parse_ast!("service.proto");
    let target_ast = vec![
        ast::RootEntry::Syntax(Cow::from("proto3")),
        ast::RootEntry::from(ast::Service::new(
            "Service",
            vec![
                ast::ServiceEntry::from(ast::Option::new(
                    "uninterpreted_option",
                    ast::MapValue::from(ast::Map::from([(
                        Cow::from("string_value"),
                        ast::MapValue::String(Cow::from("")),
                    )])),
                )),
                ast::ServiceEntry::from(ast::Rpc::new(
                    "RPC1",
                    "Request",
                    "Reply",
                    ast::RpcStream::new(false, false),
                )),
                ast::ServiceEntry::from(ast::Rpc::new(
                    "RPC2",
                    "Request",
                    "Reply",
                    ast::RpcStream::new(true, false),
                )),
                ast::ServiceEntry::from(ast::Rpc::new(
                    "RPC3",
                    "Request",
                    "Reply",
                    ast::RpcStream::new(false, true),
                )),
                ast::ServiceEntry::from(ast::Rpc::new(
                    "RPC4",
                    "Request",
                    "Reply",
                    ast::RpcStream::new(true, true),
                )),
            ],
        )),
        ast::RootEntry::from(ast::Message::empty("Request")),
        ast::RootEntry::from(ast::Message::empty("Reply")),
    ];

    assert_eq!(ast, target_ast);
}
