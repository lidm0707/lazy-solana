use dioxus::prelude::*;

use crate::theme::Theme;
#[component]
pub fn Layout(children: Element) -> Element {
    let layout_theme = use_context::<Signal<Theme>>()() == Theme::Dark;
    rsx! {
        div {
            class: format!("h-screen w-full flex flex-col {} {}",
                           if layout_theme { "dark" } else { "" },
                           if  layout_theme{ "bg-slate-900" } else { "bg-gray-100" }),
            div { class: "p-4 bg-white dark:bg-slate-800 shadow-md",
                div { class: "flex justify-between items-center",
                    h1 { class: "text-3xl font-bold text-purple-600 dark:text-purple-400", "Solana Account Graph Explorer" }
                    button {
                        class: "px-4 py-2 text-sm font-medium rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2",
                        class: if layout_theme{
                            "bg-teal-700 text-teal-100 hover:bg-teal-600 focus:ring-teal-500"
                        } else {
                            "bg-teal-100 text-teal-700 hover:bg-teal-200 focus:ring-teal-500"
                        },
                        onclick: move |_| {
                            if layout_theme {
                                use_context::<Signal<Theme>>().set(Theme::Light);
                            } else {
                                use_context::<Signal<Theme>>().set(Theme::Dark);
                            }
                        },
                         if layout_theme { "Dark Mode" } else { "Light Mode" }
                    }
                }

            }
            {children}

        }
    }
}
