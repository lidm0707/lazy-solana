use dioxus::prelude::*;
use lazry_solana::{
    components::{
        canvas_component::canvas::Canvas,
        node_component::{node::Node, node_hook::use_nodes},
        search_component::search::Search,
    },
    fecth::rpc_service::search_account,
};

const TAILWIND_CSS: Asset = asset!("./assets/output.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let nodes = use_nodes();
    let error = use_signal(|| None::<String>);
    let is_dark_mode = use_signal(|| true);
    let mut address_input = use_signal(|| "kJpCEdgKBL1T4N5zjdoe5bGn8kNFMwmX6bKmdMjSXen".to_owned());
    let mut is_dark_mode_for_toggle = is_dark_mode.to_owned();
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: format!("h-screen w-full flex flex-col {} {}",
                           if *is_dark_mode.read() { "dark" } else { "" },
                           if *is_dark_mode.read() { "bg-slate-900" } else { "bg-gray-100" }),
            div { class: "p-4 bg-white dark:bg-slate-800 shadow-md",
                div { class: "flex justify-between items-center",
                    h1 { class: "text-3xl font-bold text-purple-600 dark:text-purple-400", "Solana Account Graph Explorer" }
                    button {
                        class: "px-4 py-2 text-sm font-medium rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2",
                        class: if *is_dark_mode.read() {
                            "bg-teal-700 text-teal-100 hover:bg-teal-600 focus:ring-teal-500"
                        } else {
                            "bg-teal-100 text-teal-700 hover:bg-teal-200 focus:ring-teal-500"
                        },
                        onclick: move |_| is_dark_mode_for_toggle.set(!is_dark_mode_for_toggle.cloned()),
                        if *is_dark_mode.read() { "Light Mode" } else { "Dark Mode" }
                    }
                }
                div { class: "max-w-xl mx-auto p-6 mt-6 bg-gray-50 dark:bg-slate-700 rounded-lg shadow",
                    h2 { class: "text-xl font-semibold text-gray-700 dark:text-slate-300 mb-3 text-center", "Search Program Accounts" }
                    Search { address_input,nodes:nodes.to_owned(),error }
                }
                if let Some(error_message) = &*error.read() {
                    div { class: "max-w-xl mx-auto mt-4 p-3 bg-red-100 dark:bg-red-900/30 border border-red-400 dark:border-red-700/50 text-red-700 dark:text-red-400 rounded-md shadow",
                        "Error: {error_message}"
                    }
                }
            }

            div { class: "flex-grow p-4",
                Canvas {
                    nodes: nodes.to_owned(),
                    is_dark_mode: *is_dark_mode.read(),
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
                                            stroke: if *is_dark_mode.read() { "rgb(0, 255, 163)" } else { "rgb(156, 64, 255)" }, // Solana Green for dark, Purple for light
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
