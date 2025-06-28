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

#[component]
pub fn Home() -> Element {
    let is_dark_mode = use_context::<Signal<Theme>>()() == Theme::Dark;
    let nodes = use_nodes();
    let error = use_signal(|| None::<String>);
    let address_input = use_signal(|| "kJpCEdgKBL1T4N5zjdoe5bGn8kNFMwmX6bKmdMjSXen".to_owned());
    use_effect(move || {
        // fetch_account_info(test, "59t9zuR99FeukyeQcDdYxNLq7NFZ1SKydUxTY4sFpNC4");
        // fetch_signatures_for_address(test2, "A5wX7LrjyDHSgUbrvZkahLjbT4nb9xV94bXJ1ZmHh98M");
        // fetch_transaction(
        //     test3,
        //     "36XmiNwDjdKDxBrn1DLB4RHK8KLep1sp2J2BPpvDL6VFdi1vgKqqfrcpz2UZB1QwHEqLc3tDNV6r31ig4sBWGMSb",
        // );
    });

    rsx! {

        Layout {

            Search  { address_input,nodes:nodes.to_owned(),error }
            if let Some(error_message) = &*error.read() {
                div { class: "max-w-xl mx-auto mt-4 p-3 bg-red-100 dark:bg-red-900/30 border border-red-400 dark:border-red-700/50 text-red-700 dark:text-red-400 rounded-md shadow",
                    "Error: {error_message}"
                }
            }
            div { class: "flex-grow p-4",
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



            }
    }
    }
}
