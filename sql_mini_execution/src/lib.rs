use std::collections::HashMap;

use derive_more::Display;
use error::QueryExecutionError;
use row::Row;
use sql_mini_parser::ast::SqlQuery;
use table::Table;

pub mod error;
pub mod row;
pub mod table;

#[derive(Debug, Display)]
pub enum ExecResponse<'a> {
    #[display("{_0:?}")]
    Select(Vec<Row<'a>>),
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
                let table = select.table;
                let table = self
                    .tables
                    .get(&table)
                    .ok_or(QueryExecutionError::TableNotFound(table))?;

                let rows = table.into_iter().collect();
                Ok(ExecResponse::Select(rows))
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
