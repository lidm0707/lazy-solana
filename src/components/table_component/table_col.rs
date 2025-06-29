use reslt::prelude::*;

use crate::components::table_component::table::DataTable;

pub fn create_col() -> PropCol<DataTable> {
    PropCol {
        cols: vec![
            Col {
                head: "ADDRESS".to_string(),
                index: "addr".to_string(),
                class: None,
                action: None,
            },
            Col {
                head: "DATA".to_string(),
                index: "data".to_string(),
                class: None,
                action: None,
            },
            Col {
                head: "SPACE".to_string(),
                index: "space".to_string(),
                class: None,
                action: None,
            },
        ],
    }
}
