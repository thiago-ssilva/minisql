use derive_more::Display;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::multispace1,
    combinator::map,
    error::context,
    sequence::{preceded, separated_pair},
    Parser,
};
use serde::{Deserialize, Serialize};

use crate::parse::{comma_sep, identifier, Parse, ParseResult, RawSpan};

/// A column's type
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize, Display, Copy)]
pub enum SqlTypeInfo {
    String,
    Int,
}

// parses "string | int"
impl<'a> Parse<'a> for SqlTypeInfo {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let mut parser = context(
            "Column Type",
            // alt will try each passed parser and return what ever succeeds
            alt((
                map(tag_no_case("string"), |_| Self::String),
                map(tag_no_case("int"), |_| Self::Int),
            )),
        );

        parser.parse(input)
    }
}

/// A column's name + type
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub type_info: SqlTypeInfo,
}

// parses "<colName> <colTyle>"
impl<'a> Parse<'a> for Column {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        context(
            "Create Column",
            map(
                separated_pair(
                    context("Column Name", identifier),
                    multispace1,
                    SqlTypeInfo::parse,
                ),
                |(name, type_info)| Self { name, type_info },
            ),
        )
        .parse(input)
    }
}

// The table and its column to create
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CreateStatement {
    pub table: String,
    pub columns: Vec<Column>,
}

//parses a comma seperated list of column and definitions contained in parens
//fn column_definitions(input: RawSpan<'_>) -> ParseResult<'_, Vec<Column>> {
//    let mut parser = context(
//        "Column Definitions",
//        map(
//            delimited(tag("("), comma_sep(Column::parse), tag(")")),
//            |(_, cols, _)| cols,
//        ),
//    );
//
//    //parser.parse(input)
//    let test: Vec<Column> = Vec::new();
//    Ok(test)
//}
fn column_definitions(input: RawSpan<'_>) -> ParseResult<'_, Vec<Column>> {
    let mut parser = context(
        "Column Definitions",
        map(
            (tag("("), comma_sep(Column::parse), tag(")")),
            |(_, cols, _)| cols,
        ),
    );

    parser.parse(input)
}

//Parses "CREATE TABLE <table name> <column defs>"
impl<'a> Parse<'a> for CreateStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let mut parser = map(
            context(
                "Create Table",
                separated_pair(
                    // table name
                    preceded(
                        (
                            tag_no_case("create"),
                            multispace1,
                            tag_no_case("table"),
                            multispace1,
                        ),
                        context("Table Name", identifier),
                    ),
                    multispace1,
                    // column defs
                    column_definitions,
                ),
            ),
            |(table, columns)| Self { table, columns },
        );

        parser.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::Parse;

    use super::{Column, CreateStatement, SqlTypeInfo};

    #[test]
    fn test_create() {
        let expeted = CreateStatement {
            table: "foo".into(),
            columns: vec![
                Column {
                    name: "col1".into(),
                    type_info: SqlTypeInfo::Int,
                },
                Column {
                    name: "col2".into(),
                    type_info: SqlTypeInfo::String,
                },
                Column {
                    name: "col3".into(),
                    type_info: SqlTypeInfo::String,
                },
            ],
        };

        assert_eq!(
            CreateStatement::parse_from_raw(
                "CREATE TABLE foo (col1 int, col2 string, col3 string)"
            )
            .unwrap()
            .1,
            expeted
        )
    }
}
