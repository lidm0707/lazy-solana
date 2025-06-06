use dioxus::prelude::*;
use lazry_solana::{
    components::{
        canvas_component::canvas::Canvas,
        node_component::{
            node::Node,
            // AccountInfo is removed as it's no longer directly used in this file
            node_hook::{PropNodes, use_nodes},
        },
    },
    // Import the search_account function from its new location
    fecth::rpc_service::search_account,
};
// Removed reqwest, serde_json, and HashMap imports as they are no longer needed here

const TAILWIND_CSS: Asset = asset!("./assets/output.css");

fn main() {
    dioxus::launch(App);
}

// search_account function has been moved to frontend/src/fecth/rpc_service.rs
// extract_accounts_data function has been moved to frontend/src/fecth/rpc_service.rs

#[component]
fn App() -> Element {
    let nodes = use_nodes();
    let error = use_signal(|| None::<String>);
    let is_dark_mode = use_signal(|| true); // Default to dark mode
    let mut address_input = use_signal(|| "kJpCEdgKBL1T4N5zjdoe5bGn8kNFMwmX6bKmdMjSXen".to_owned());
    let node_onkeypress = nodes.to_owned();
    let mut is_dark_mode_for_toggle = is_dark_mode.to_owned(); // Clone for the toggle button
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: format!("h-screen w-full flex flex-col {} {}",
                           if *is_dark_mode.read() { "dark" } else { "" },
                           if *is_dark_mode.read() { "bg-slate-900" } else { "bg-gray-100" }),
            // Header and Search Section
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
                    div { class: "flex flex-col sm:flex-row gap-3 items-center",
                        input {
                            class: "flex-grow px-4 py-2 border border-gray-300 dark:border-slate-600 bg-white dark:bg-slate-800 text-gray-900 dark:text-slate-200 dark:placeholder-slate-400 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500 dark:focus:ring-pink-500 focus:border-transparent संक्रमण-colors duration-200",
                            r#type: "text",
                            placeholder: "Enter Program ID...",
                            value: "{address_input.read()}",
                            oninput: move |e| address_input.set(e.value()),
                            onkeypress: move |e| {
                                if e.key() == Key::Enter {
                                    let address = address_input.read().clone();
                                    if !address.trim().is_empty() {
                                        // Call the imported search_account function
                                        search_account(
                                            address,
                                            node_onkeypress.to_owned(),
                                            error,
                                        );
                                    }
                                }
                            },
                        }
                        button {
                            class: "px-6 py-3 bg-purple-600 text-white font-semibold rounded-md hover:bg-purple-700 dark:bg-purple-500 dark:hover:bg-purple-600 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-opacity-50 disabled:bg-gray-400 dark:disabled:bg-slate-600 संक्रमण-colors duration-200 w-full sm:w-auto",
                            disabled: address_input.read().trim().is_empty(),
                            onclick: move |_| {
                                let address = address_input.read().clone();
                                if !address.trim().is_empty() {
                                    // Call the imported search_account function
                                    search_account(
                                        address,
                                        nodes.to_owned(),
                                        error,
                                    );
                                }
                            },
                            {"Search"}
                        }
                    }
                }
                // Error Display Section
                if let Some(error_message) = &*error.read() {
                    div { class: "max-w-xl mx-auto mt-4 p-3 bg-red-100 dark:bg-red-900/30 border border-red-400 dark:border-red-700/50 text-red-700 dark:text-red-400 rounded-md shadow",
                        "Error: {error_message}"
                    }
                }
            }

            // Canvas Section - takes remaining space
            div { class: "flex-grow p-4", // Background will be inherited from the main 'dark' div
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
// Render edges
