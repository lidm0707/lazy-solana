use std::fmt::Debug;

use dioxus::prelude::*;
use reslt::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq, Clone, FieldAccessible)]
pub struct DataTable {
    pub addr: String,
    pub data: String,
    pub space: String,
}

#[component]
pub fn CustomeTable(use_table: UseTable<DataTable>) -> Element {
    let contable_config = TableConfig::default()
        .set_table_container("absolute overflow-auto h-150 w-1/2".to_string());
    rsx! {    DefaultTable {class:contable_config,table:use_table}}
}
