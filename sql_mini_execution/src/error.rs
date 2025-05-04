use sql_mini_parser::{commands::SqlTypeInfo, value::Value};
use thiserror::Error;

/// Query exectuion errors

#[derive(Error, Debug)]
#[error("Query Execution Error")]
pub enum QueryExecutionError {
    #[error("Table {0} was not found")]
    TableNotFound(String),

    #[error("Table {0} already exists")]
    TAbleAlreadyExists(String),

    #[error("Column {0} does not exist")]
    ColumnDoesNotExist(String),

    #[error("Value {1} can not be inserted into a {0} column")]
    InsertTypeMismatch(SqlTypeInfo, Value),
}
