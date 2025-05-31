use dioxus::prelude::*;
use frontend::{
    components::{
        canvas_component::canvas::Canvas,
        node_component::{
            node::Node,
            node_hook::{use_nodes, AccountInfo, PropNodes},
        },
    },
    data::node_data::{DataNode, IntoDataNodes},
};

const TAILWIND_CSS: Asset = asset!("./assets/output.css");

fn main() {
    dioxus::launch(App);
}

pub fn search_account<T>(
    _fecth_struct: T,
    address_input: String,
    nodes: PropNodes,
    is_loading: Signal<bool>,
    error: Signal<Option<String>>,
) where
    T: IntoDataNodes + 'static,
{
    spawn({
        let mut error = error.to_owned();
        let nodes = nodes.to_owned();
        let mut is_loading_clone = is_loading.to_owned();
        let address_input = address_input.to_owned();
        let client = T::new(address_input.clone());
        async move {
            is_loading_clone.set(true);
            let client = client.get_accounts().await;
            let mut nodes = nodes.to_owned();

            match client {
                Ok(response) => match response.json::<Vec<AccountInfo>>().await {
                    Ok(fetched) => {
                        nodes.set_prop_nodes(address_input.clone(), fetched);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to parse search results: {}", e)));
                    }
                },
                Err(e) => {
                    error.set(Some(format!("Search request failed: {}", e)));
                }
            }

            is_loading_clone.set(false);
        }
    });
}

#[component]
fn App() -> Element {
    let nodes = use_nodes();
    let accounts = use_signal(|| Vec::<AccountInfo>::new());
    let error = use_signal(|| None::<String>);
    let is_loading = use_signal(|| false);
    let mut address_input = use_signal(|| "kJpCEdgKBL1T4N5zjdoe5bGn8kNFMwmX6bKmdMjSXen".to_owned());
    let data_node = DataNode::new(address_input.cloned());

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "h-screen w-full bg-gray-100",
            div { class: "p-4",
                h1 { class: "text-2xl font-bold mb-4", "Account Graph" }
                // Search form
                div { class: "mb-4 p-4 bg-white rounded shadow",
                    h2 { class: "text-lg font-semibold mb-2", "Search Account" }
                    div { class: "flex gap-2",
                        {
                            let nodes = nodes.to_owned();
                            let data_node = data_node.to_owned();
                            rsx! {
                                input {
                                    class: "flex-1 px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    r#type: "text",
                                    placeholder: "Enter wallet address...",
                                    value: "{address_input.read()}",
                                    oninput: move |e| address_input.set(e.value()),
                                    onkeypress: move |e| {
                                        if e.key() == Key::Enter {
                                            let address = address_input.read().clone();
                                            if !address.trim().is_empty() {
                                                search_account(
                                                    data_node.to_owned(),
                                                    address,
                                                    nodes.to_owned(),
                                                    is_loading,
                                                    error,
                                                );
                                            }
                                        }
                                    },
                                }
                            }
                        }

                        button {
                            class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-400",
                            disabled: *is_loading.read() || address_input.read().trim().is_empty(),
                            onclick: move |_| {
                                let address = address_input.read().clone();
                                if !address.trim().is_empty() {
                                    search_account(
                                        data_node.to_owned(),
                                        address,
                                        nodes.to_owned(),
                                        is_loading,
                                        error,
                                    );
                                }
                            },
                            if *is_loading.read() {
                                "Searching..."
                            } else {
                                "Search"
                            }
                        }
                    }
                }
                // Error display
                if let Some(error_message) = &*error.read() {
                    div { class: "text-red-500 mb-4 p-3 bg-red-50 rounded", "Error: {error_message}" }
                }
                // Account info panel
                div { class: "mb-4 p-4 bg-white rounded shadow",
                    h2 { class: "text-lg font-semibold mb-2",
                        "Search Results ({accounts.read().len()} accounts)"
                    }
                    if accounts.read().is_empty() && !*is_loading.read() {
                        div { class: "text-gray-500 italic",
                            "Enter an address above to search for account information"
                        }
                    } else {
                        div { class: "max-h-32 overflow-y-auto",
                            for account in accounts.read().iter() {
                                div { class: "text-sm text-gray-600 truncate mb-1",
                                    "Pubkey: {account.pubkey.chars().take(8).collect::<String>()}... | Lamports: {account.lamports}"
                                }
                            }
                        }
                    }
                }
            }
            // SVG Canvas
            Canvas {
                nodes: nodes.to_owned(),
                child: rsx! {
                    for edge in nodes.to_owned().list_edges.read().iter() {
                        {
                            let nodes = nodes.to_owned();
                            let from = nodes
                                .list_nodes
                                .read()
                                .clone()
                                .into_iter()
                                .find(|n| n.id == edge.from);
                            let to = nodes.list_nodes.read().clone().into_iter().find(|n| n.id == edge.to);
                            if let (Some(from), Some(to)) = (from, to) {
                                rsx! {
                                    {println!("is line")}
                                    line {
                                        x1: "{from.x}",
                                        y1: "{from.y}",
                                        x2: "{to.x}",
                                        y2: "{to.y}",
                                        stroke: "black",
                                        stroke_width: "3",
                                    }
                                }
                            } else {
                                rsx! { "no line" }
                            }
                        }
                    }
                    
                    for node in nodes.to_owned().list_nodes.read().to_owned().into_iter() {
                        Node { prop: node.to_owned(), id: node.id }
                    }
                },
            }
        }
    }
}
// Render edges
