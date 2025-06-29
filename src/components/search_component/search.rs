use crate::{components::node_component::node_hook::PropNodes, idl_helper::idl::Idl};
use dioxus::prelude::*;
use reslt::prelude::*;
#[component]
pub fn ModalWord() -> Element {
    let mut word: Signal<String> = use_context();
    rsx! {
        div { class:"overflow",
            button { class: "px-6 py-3 bg-purple-600 text-white font-semibold rounded-md hover:bg-purple-700 dark:bg-purple-500 dark:hover:bg-purple-600 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-opacity-50 disabled:bg-gray-400 dark:disabled:bg-slate-600 संक्रमण-colors w-full sm:w-auto",
                onclick: move |_| {
                   use_context::<UseModal>().close();
                    println!("test close");
                }
            },
            form {
                label {
                    for: "name-input",
                    "Name"
                }
                textarea {
                    class:"h-auto block w-full resize-y border rounded-md py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                    id: "name-input",
                    value: "{word}",
                    oninput: move |evt| word.set(evt.value()),
                    name: "name",
                }
            }
        }


    }
}
#[component]
pub fn Search(
    address_input: Signal<String>,
    nodes: PropNodes,
    error: Signal<Option<String>>,
) -> Element {
    rsx! {
        div { class: "max-w-xl mx-auto p-6 mt-6 bg-gray-50 dark:bg-slate-700 rounded-lg shadow",
            h2 { class: "text-xl font-semibold text-gray-700 dark:text-slate-300 mb-3 text-center", "Search Program Accounts" }
            div { class: "flex flex-col sm:flex-row gap-3 items-center",


                button { class: "px-6 py-3 bg-purple-600 text-white font-semibold rounded-md hover:bg-purple-700 dark:bg-purple-500 dark:hover:bg-purple-600 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-opacity-50 disabled:bg-gray-400 dark:disabled:bg-slate-600 संक्रमण-colors duration-200 w-full sm:w-auto",
                    onclick: move |_| {
                        use_context::<UseModal>().set_title("Modal String");
                        use_context::<UseModal>().set_content(rsx!{ModalWord{}});
                        use_context::<UseModal>().open();
                    },
                    {"PASTE IDL"}
                }


                button { class: "px-6 py-3 bg-purple-600 text-white font-semibold rounded-md hover:bg-purple-700 dark:bg-purple-500 dark:hover:bg-purple-600 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-opacity-50 disabled:bg-gray-400 dark:disabled:bg-slate-600 संक्रमण-colors duration-200 w-full sm:w-auto",
                    onclick: move |_| {
                        let   word: Signal<String> = use_context();
                        // search_account(word(),node_onkeypress.to_owned(),error.to_owned());
                        let  idl  = Idl::new(word());

                        let accounts = idl.get_idl_idl_node();


                        nodes.set_prop_nodes(idl.address,accounts);
                        println!("{:?}",nodes);



                    },
                    {"SEARCH"}
                }

            }
        }


    }
}
