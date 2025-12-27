use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub proto
);

pub mod ast;
pub mod lexer;

#[cfg(test)]
mod tests;

#[allow(clippy::needless_lifetimes)]
pub fn parse<'a>(
    data: &'a str,
) -> Result<ast::File<'a>, lalrpop_util::ParseError<usize, lexer::Token<'a>, lexer::LexicalError<'a>>>
{
    let lexer = lexer::Lexer::new(data);
    let parser = proto::FileParser::new();

    parser.parse(data, lexer)
}
