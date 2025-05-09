use std::collections::HashMap;

use sql_mini_parser::value::Value;

use crate::error::QueryExecutionError;

#[derive(Debug, Clone)]
pub struct Row<'a> {
    id: usize,
    data: HashMap<&'a String, &'a Value>,
}

impl<'a> Row<'a> {
    pub fn new(id: usize, data: HashMap<&'a String, &'a Value>) -> Self {
        Self { id, data }
    }

    pub fn get(&self, column: &String) -> Result<Value, QueryExecutionError> {
        self.data.get(&column).map_or_else(
            || Err(QueryExecutionError::ColumnDoesNotExist(column.to_owned())),
            |val| Ok((*val).clone()),
        )
    }
}
