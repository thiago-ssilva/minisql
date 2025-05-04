use std::str::FromStr;

use bigdecimal::BigDecimal;
use derive_more::Display;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::multispace0,
    error::context,
    sequence::{preceded, terminated},
    Parser,
};
use serde::{Deserialize, Serialize};

use crate::parse::{peek_then_cut, Parse, ParseResult, RawSpan};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display)]
pub enum Value {
    Number(BigDecimal),
    String(String),
}

///Parse a single quoted string value
fn parse_string_value(input: RawSpan<'_>) -> ParseResult<'_, Value> {
    let (remaining_input, (_, str_value, _)) = context(
        "String Literal",
        (
            tag("'"),
            take_until("'").map(|s: RawSpan| Value::String(s.fragment().to_string())),
            tag("'"), // take_until does not consume the ending quote
        ),
    )
    .parse(input)?;

    Ok((remaining_input, str_value))
}

/// Parse a numeric literal
fn parse_number_value(input: RawSpan<'_>) -> ParseResult<'_, Value> {
    let (reamining_input, digits) =
        context("Number Literal", take_while(|c: char| c.is_numeric())).parse(input)?;

    Ok((
        reamining_input,
        Value::Number(BigDecimal::from_str(&digits).unwrap()),
    ))
}

impl<'a> Parse<'a> for Value {
    fn parse(input: crate::parse::RawSpan<'a>) -> crate::parse::ParseResult<'a, Self> {
        let mut parser = context(
            "Value",
            preceded(
                multispace0,
                terminated(
                    alt((peek_then_cut("'", parse_string_value), parse_number_value)),
                    multispace0,
                ),
            ),
        );

        parser.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bigdecimal::BigDecimal;

    use crate::parse::Parse;

    use super::Value;

    #[test]
    fn test_string() {
        let expected = Value::String(String::from("123 abc new"));
        let expected_remaining = String::from("random '123'");

        let (remaining_input, value) = Value::parse_from_raw("'123 abc new' random '123'").unwrap();

        assert_eq!(value, expected);

        assert_eq!(remaining_input.to_string(), expected_remaining);
    }

    #[test]
    fn test_number() {
        let expected = Value::Number(BigDecimal::from_str(&String::from("10000")).unwrap());
        let expected_remaining = String::from("rest of string");

        let (remaining_input, value) = Value::parse_from_raw("10000 rest of string").unwrap();

        assert_eq!(value, expected);

        assert_eq!(remaining_input.to_string(), expected_remaining);
    }
}
