use std::pin::Pin;

use crate::{
    components::{
        canvas_component::canvas::Canvas,
        node_component::{node::Node, node_hook::use_nodes},
        search_component::search::Search,
    },
    pages::home::layout::Layout,
    theme::Theme,
};
use dioxus::prelude::*;
use reslt::prelude::*;
use serde::Serialize;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, FieldAccessible)]
pub struct MyData {
    pub id: u64,
    pub name: String,
    pub style: String,
    pub description: String,
    pub details: String,
    pub details_style: String,
}

pub fn create_col() -> PropCol<MyData> {
    PropCol {
        cols: vec![
            Col {
                head: "ID".to_owned(),
                index: "id".to_owned(),
                class: Some("text-right".to_owned()),
                action: None,
            },
            Col {
                head: "Name".to_owned(),
                index: "name".to_owned(),
                class: None,
                action: None,
            },
            Col {
                head: "Style".to_owned(),
                index: "style".to_owned(),
                class: None,
                action: None,
            },
            Col {
                head: "Description".to_owned(),
                index: "description".to_owned(),
                class: None,
                action: None,
            },
            Col {
                head: "Details".to_owned(),
                index: "details".to_owned(),
                class: None,
                action: None,
            },
            Col {
                head: "Details Style".to_owned(),
                index: "details_style".to_owned(),
                class: None,
                action: None,
            },
        ],
    }
}

pub fn get_person_data(
    _start: usize,
    _end: usize,
    _sort: (String, bool),
) -> Pin<Box<dyn 'static + Future<Output = (PropData<MyData>, usize)>>> {
    Box::pin(async move {
        let data = vec![
            MyData {
                id: 1,
                name: "John Doe".to_owned(),
                style: "style1".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style1".to_owned(),
            },
            MyData {
                id: 2,
                name: "Jane Doe".to_owned(),
                style: "style2".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style2".to_owned(),
            },
            MyData {
                id: 3,
                name: "John Smith".to_owned(),
                style: "style1".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style1".to_owned(),
            },
            MyData {
                id: 4,
                name: "Jane Smith".to_owned(),
                style: "style2".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style2".to_owned(),
            },
            MyData {
                id: 5,
                name: "John Doe".to_owned(),
                style: "style1".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style1".to_owned(),
            },
            MyData {
                id: 6,
                name: "Jane Doe".to_owned(),
                style: "style2".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style2".to_owned(),
            },
            MyData {
                id: 7,
                name: "John Smith".to_owned(),
                style: "style1".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style1".to_owned(),
            },
            MyData {
                id: 8,
                name: "Jane Smith".to_owned(),
                style: "style2".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style2".to_owned(),
            },
            MyData {
                id: 9,
                name: "John Doe".to_owned(),
                style: "style1".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style1".to_owned(),
            },
            MyData {
                id: 10,
                name: "Jane Doe".to_owned(),
                style: "style2".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style2".to_owned(),
            },
            MyData {
                id: 11,
                name: "John Smith".to_owned(),
                style: "style1".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style1".to_owned(),
            },
            MyData {
                id: 12,
                name: "Jane Smith".to_owned(),
                style: "style2".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style2".to_owned(),
            },
            MyData {
                id: 13,
                name: "John Doe".to_owned(),
                style: "style1".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style1".to_owned(),
            },
            MyData {
                id: 14,
                name: "Jane Doe".to_owned(),
                style: "style2".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style2".to_owned(),
            },
            MyData {
                id: 15,
                name: "John Smith".to_owned(),
                style: "style1".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style1".to_owned(),
            },
            MyData {
                id: 16,
                name: "Jane Smith".to_owned(),
                style: "style2".to_owned(),
                description: "Description".to_owned(),
                details: "Details".to_owned(),
                details_style: "style2".to_owned(),
            },
        ];
        (
            PropData {
                data_vec: data.clone(),
            },
            data.len(),
        )
    })
}

#[component]
pub fn Home() -> Element {
    let is_dark_mode = use_context::<Signal<Theme>>()() == Theme::Dark;
    let nodes = use_nodes();
    let error = use_signal(|| None::<String>);
    // Your mock fetch_fn

    let table = use_table::<MyData>(get_person_data, create_col(), None);
    use_effect(move || {
        // fetch_account_info(test, "59t9zuR99FeukyeQcDdYxNLq7NFZ1SKydUxTY4sFpNC4");
        // fetch_signatures_for_address(test2, "A5wX7LrjyDHSgUbrvZkahLjbT4nb9xV94bXJ1ZmHh98M");
        // fetch_transaction(
        //     test3,
        //     "36XmiNwDjdKDxBrn1DLB4RHK8KLep1sp2J2BPpvDL6VFdi1vgKqqfrcpz2UZB1QwHEqLc3tDNV6r31ig4sBWGMSb",
        // );
    });
    let contable_config = TableConfig::default()
        .set_table_container("absolute overflow-auto h-150 w-1/2".to_string());

    rsx! {

        Layout {

            Search  { nodes:nodes.to_owned(),error }
            if let Some(error_message) = &*error.read() {
                div { class: "max-w-xl mx-auto mt-4 p-3 bg-red-100 dark:bg-red-900/30 border border-red-400 dark:border-red-700/50 text-red-700 dark:text-red-400 rounded-md shadow",
                    "Error: {error_message}"
                }
            }
            div { class: "flex p-4",
                Canvas {
                    nodes: nodes.to_owned(),
                    child: rsx! {
                        for edge in nodes.to_owned().list_edges.read().iter() {
                            {
                                let current_nodes = nodes.list_nodes.read();
                                let from_node = current_nodes.iter().find(|n| n.id == edge.from);
                                let to_node = current_nodes.iter().find(|n| n.id == edge.to);

                                if let (Some(from), Some(to)) = (from_node, to_node) {
                                    rsx! {
                                        line {
                                            x1: "{from.x}",
                                            y1: "{from.y}",
                                            x2: "{to.x}",
                                            y2: "{to.y}",
                                            stroke: if is_dark_mode { "rgb(0, 255, 163)" } else { "rgb(156, 64, 255)" }, // Solana Green for dark, Purple for light
                                            stroke_width: "2.5",
                                        }
                                    }
                                } else {
                                    rsx!{}
                                }
                            }
                        }

                        for node_data in nodes.to_owned().list_nodes.read().iter() {
                            Node { prop: node_data.clone(), id: node_data.id }
                        }
                    },


                }
                div{class:" ml-10",DefaultTable {class:contable_config,table  }}



            }
    }
    }
}
