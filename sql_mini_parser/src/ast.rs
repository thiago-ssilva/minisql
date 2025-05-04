use nom::{
    branch::alt,
    character::complete::{char, multispace0},
    combinator::map,
    error::context,
    sequence::preceded,
    Parser,
};
use serde::{Deserialize, Serialize};

use crate::{
    commands::{CreateStatement, InsertStatement, SelectStatement},
    parse::{peek_then_cut, Parse},
};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum SqlQuery {
    Select(SelectStatement),
    Create(CreateStatement),
    Insert(InsertStatement),
}

impl<'a> Parse<'a> for SqlQuery {
    fn parse(input: crate::parse::RawSpan<'a>) -> crate::parse::ParseResult<'a, Self> {
        let (remaining_input, (query, _, _, _)) = context(
            "Query",
            preceded(
                multispace0,
                (
                    alt((
                        peek_then_cut("select", map(SelectStatement::parse, SqlQuery::Select)),
                        peek_then_cut("create", map(CreateStatement::parse, SqlQuery::Create)),
                        peek_then_cut("insert", map(InsertStatement::parse, SqlQuery::Insert)),
                    )),
                    multispace0,
                    char(';'),
                    multispace0,
                ),
            ),
        )
        .parse(input)?;

        Ok((remaining_input, query))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::{Column, SqlTypeInfo},
        value::Value,
    };

    use super::*;

    #[test]
    fn test_error() {
        let query = SqlQuery::parse_from_raw("select fart;");
        assert!(query.is_err(), "expected parse to fail, got {query:?}");
    }

    #[test]
    fn test_select() {
        let expected = SelectStatement {
            table: String::from("users"),
            fields: vec![String::from("name"), String::from("id")],
        };

        assert_eq!(
            SqlQuery::parse_from_raw("select name, id from users;")
                .unwrap()
                .1,
            SqlQuery::Select(expected)
        );
    }

    #[test]
    fn test_insert() {
        let expected = InsertStatement {
            table: String::from("users"),
            values: vec![
                Value::String(String::from("john")),
                Value::String(String::from("jane")),
            ],
        };

        assert_eq!(
            SqlQuery::parse_from_raw("insert into users values 'john', 'jane';")
                .unwrap()
                .1,
            SqlQuery::Insert(expected)
        );
    }
    #[test]
    fn test_create() {
        let expected = CreateStatement {
            table: String::from("users"),
            columns: vec![
                Column {
                    name: String::from("name"),
                    type_info: SqlTypeInfo::String,
                },
                Column {
                    name: String::from("age"),
                    type_info: SqlTypeInfo::Int,
                },
            ],
        };

        assert_eq!(
            SqlQuery::parse_from_raw("CREATE TABLE users (name string, age int);")
                .unwrap()
                .1,
            SqlQuery::Create(expected)
        );
    }
}
