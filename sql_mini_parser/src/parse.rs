use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::multispace0,
    combinator::map,
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};
use nom_locate::LocatedSpan;

//Use nom_locate's LocatedSpan as a wrapper around a string input
pub type RawSpan<'a> = LocatedSpan<&'a str>;

//The result for all of our parsers, they will have our span type as input and can have any output
// this will use a default error type but we will change that latter
pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T>;

/// Parse a unquoted sql identifier
pub(crate) fn identifier(i: RawSpan) -> ParseResult<String> {
    map(take_while1(|c: char| c.is_alphanumeric()), |s: RawSpan| {
        s.fragment().to_string()
    })
    .parse(i)
}

/// Implement the parse function to more easily convert a span into a sql
/// command
pub trait Parse<'a>: Sized {
    /// Parse the given span into self
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self>;

    /// Helper method for tests to convert a str into a raw span and parse
    fn parse_from_raw(input: &'a str) -> ParseResult<'a, Self> {
        let i = LocatedSpan::new(input);
        Self::parse(i)
    }
}

pub(crate) fn comma_sep<'a, O, E, F>(
    f: F,
) -> impl nom::Parser<RawSpan<'a>, Output = Vec<O>, Error = E>
where
    F: nom::Parser<RawSpan<'a>, Error = E, Output = O>,
    E: nom::error::ParseError<RawSpan<'a>>,
{
    let separator = delimited(multispace0, tag(","), multispace0);
    separated_list1(separator, f)
}
