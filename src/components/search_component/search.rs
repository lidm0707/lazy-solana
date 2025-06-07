use crate::{components::node_component::node_hook::PropNodes, fecth::rpc_service::search_account};
use dioxus::prelude::*;

#[component]
pub fn Search(
    address_input: Signal<String>,
    nodes: PropNodes,
    error: Signal<Option<String>>,
) -> Element {
    let node_onkeypress = nodes.to_owned();
    rsx! {
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


    }
}
