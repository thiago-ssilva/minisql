use std::{collections::HashMap, rc::Rc};

use sql_mini_parser::value::Value;

use crate::table::ColumnInfo;

#[derive(Debug, Clone)]
pub struct Row<'a> {
    id: usize,
    columns: Rc<ColumnInfo>,
    data: HashMap<&'a String, &'a Value>,
}

impl<'a> Row<'a> {
    pub fn new(columns: Rc<ColumnInfo>, id: usize, data: HashMap<&'a String, &'a Value>) -> Self {
        Self { id, columns, data }
    }
}
