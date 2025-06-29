use dioxus::{logger::tracing, prelude::*};
use tracing::Level;

use lazry_solana::{
    components::table_component::table_data::AccountDetail, pages::home::page::Home, theme::Theme,
};
use reslt::prelude::*;

fn main() {
    let _ = dioxus_logger::init(Level::DEBUG);
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    const TAILWIND_CSS: Asset = asset!("./assets/output.css");
    // let word = use_signal(|| "kJpCEdgKBL1T4N5zjdoe5bGn8kNFMwmX6bKmdMjSXen".to_owned());
    use_context_provider(|| {
        Signal::new(AccountDetail {
            programe_addr: "".to_string(),
            discriminator: vec![],
            name_account: "".to_string(),
            fields: vec![],
        })
    });
    use_context_provider(|| Signal::new(Theme::Dark));

    let raw = r#"
        {
          "address": "8R5SnEabSDANPR3gtqwJDsrWDKZeuRwgjXohHBvvwuHp",
          "metadata": {
            "name": "play",
            "version": "0.1.0",
            "spec": "0.1.0",
            "description": "Created with Anchor"
          },
          "instructions": [
            {
              "name": "increment",
              "discriminator": [11, 18, 104, 9, 104, 174, 59, 33],
              "accounts": [
                {
                  "name": "counter",
                  "writable": true
                }
              ],
              "args": []
            },
            {
              "name": "initialize",
              "discriminator": [175, 175, 109, 31, 13, 152, 155, 237],
              "accounts": [
                {
                  "name": "counter",
                  "writable": true,
                  "signer": true
                },
                {
                  "name": "user",
                  "writable": true,
                  "signer": true
                },
                {
                  "name": "system_program",
                  "address": "11111111111111111111111111111111"
                }
              ],
              "args": [
                {
                  "name": "name",
                  "type": "string"
                }
              ]
            }
          ],
          "accounts": [
            {
              "name": "Counter",
              "discriminator": [255, 176, 4, 245, 188, 253, 124, 25]
            }
          ],
          "types": [
            {
              "name": "Counter",
              "type": {
                "kind": "struct",
                "fields": [
                  {
                    "name": "count",
                    "type": "u64"
                  },
                  {
                    "name": "name",
                    "type": "string"
                  }
                ]
              }
            }
          ]
        }
    "#
    .to_string();
    let word = use_signal(|| raw);
    provide_context(word);
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        ToastContainer{Modal{Home{}}}

    }
}
