use std::collections::HashMap;

use derive_more::Display;
use error::QueryExecutionError;
use sql_mini_parser::ast::SqlQuery;
use table::{Table, TableIter};

pub mod error;
pub mod row;
pub mod table;

#[derive(Debug, Display)]
pub enum ExecResponse<'a> {
    #[display("{_0:?}")]
    Select(TableIter<'a>),
    Insert,
    Create,
}

#[derive(Debug, Default)]
pub struct Execution {
    tables: HashMap<String, Table>,
}

impl Execution {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn run(&mut self, query: SqlQuery) -> Result<ExecResponse, QueryExecutionError> {
        match query {
            SqlQuery::Select(select) => {
                let columns = select.fields;
                let table = select.table;
                let table = self
                    .tables
                    .get(&table)
                    .ok_or(QueryExecutionError::TableNotFound(table))?;

                Ok(ExecResponse::Select(table.select(columns)?))
            }
            SqlQuery::Insert(insert) => {
                let Some(table) = self.tables.get_mut(&insert.table) else {
                    return Err(QueryExecutionError::TableNotFound(insert.table));
                };

                table.insert(insert.values)?;
                Ok(ExecResponse::Insert)
            }
            SqlQuery::Create(create) => {
                let table = Table::new(create.columns);
                self.tables.insert(create.table, table);
                Ok(ExecResponse::Create)
            }
        }
    }
}
