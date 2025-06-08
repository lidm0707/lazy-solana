use dioxus::prelude::*;
use lazry_solana::{pages::home::page::Home, theme::Theme};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    const TAILWIND_CSS: Asset = asset!("./assets/output.css");

    use_context_provider(|| Signal::new(Theme::Dark));
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            Home{}
        }
    }
}
