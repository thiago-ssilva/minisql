use nom::{bytes::complete::tag_no_case, character::complete::multispace1, error::context, Parser};
use serde::{Deserialize, Serialize};

use crate::parse::{comma_sep, identifier, Parse};

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SelectStatement {
    pub table: String,
    pub fields: Vec<String>,
}

//TODO: impl display

impl<'a> Parse<'a> for SelectStatement {
    fn parse(input: crate::parse::RawSpan<'a>) -> crate::parse::ParseResult<'a, Self> {
        let (remaining_input, (_, _, fields, _, _, _, table)) = context(
            "Select Statement",
            (
                tag_no_case("select"),
                multispace1,
                context("Select Columns", comma_sep(identifier)),
                multispace1,
                tag_no_case("from"),
                multispace1,
                context("From Table", identifier),
            ),
        )
        .parse(input)?;

        Ok((remaining_input, SelectStatement { fields, table }))
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::Parse;

    use super::SelectStatement;

    #[test]
    fn test_select() {
        let expected = SelectStatement {
            table: String::from("users"),
            fields: vec![String::from("name"), String::from("id")],
        };

        let value = SelectStatement::parse_from_raw("SELECT name, id FROM users");

        assert_eq!(value.unwrap().1, expected);
    }
}
