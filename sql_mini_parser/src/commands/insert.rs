use nom::{
    bytes::complete::tag_no_case, character::complete::multispace1, error::context,
    sequence::preceded, Parser,
};
use serde::{Deserialize, Serialize};

use crate::{
    parse::{comma_sep, identifier, Parse},
    value::Value,
};

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InsertStatement {
    pub table: String,
    pub values: Vec<Value>,
}

impl<'a> Parse<'a> for InsertStatement {
    fn parse(input: crate::parse::RawSpan<'a>) -> crate::parse::ParseResult<'a, Self> {
        let (remaining_input, (_, _, table, _, values)) = context(
            "Insert Statement",
            (
                tag_no_case("insert"),
                preceded(multispace1, tag_no_case("into")),
                preceded(multispace1, context("Table Name", identifier)),
                preceded(multispace1, tag_no_case("values")),
                preceded(multispace1, context("Values", comma_sep(Value::parse))),
            ),
        )
        .parse(input)?;

        Ok((remaining_input, InsertStatement { table, values }))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse::Parse, value::Value};

    use super::InsertStatement;

    #[test]
    fn test_insert() {
        let expected = InsertStatement {
            table: String::from("users"),
            values: vec![
                Value::String(String::from("john")),
                Value::String(String::from("jane")),
            ],
        };

        let (_, command) =
            InsertStatement::parse_from_raw("insert into users values 'john', 'jane'").unwrap();

        assert_eq!(command, expected);
    }
}
