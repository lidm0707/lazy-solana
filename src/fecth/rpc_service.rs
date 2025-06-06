use dioxus::prelude::*;
use reqwest::Client;
use serde_json::json;

use crate::components::node_component::node_hook::{AccountInfo, PropNodes};

pub fn search_account(
    address_input: String,
    nodes: PropNodes,
    error: Signal<Option<String>>,
) {
    spawn(async move {
        let mut error = error.to_owned();
        let mut nodes = nodes.to_owned();
        let address_input = address_input.to_owned();

        let client = Client::new();
        let rpc_url = "https://api.devnet.solana.com"; // *** REPLACE THIS WITH YOUR SOLANA RPC URL ***

        let request_body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getProgramAccounts",
            "params": [
                address_input.clone(), // Use the address input from the user as the Program ID
                {
                    "encoding": "jsonParsed", // Request data in jsonParsed format
                    "filters": [] // Changed filters to an empty array to match curl
                }
            ]
        });

        let response = client.post(rpc_url).json(&request_body).send().await;

        match response {
            Ok(res) => {
                // Use `mut res` to allow getting text later if needed
                if res.status().is_success() {
                    match res.json::<serde_json::Value>().await {
                        Ok(json) => {
                            let fetched = extract_accounts_data(&json);
                            // Clone address_input for set_prop_nodes
                            nodes.set_prop_nodes(address_input.clone(), fetched);
                        }
                        Err(err) => {
                            error.set(Some(format!("Failed to parse response JSON: {}", err)));
                        }
                    }
                } else {
                    // Get text body only if status is not successful
                    let status = res.status(); // Save status before consuming `res` for text
                    let err_msg = res
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error getting response text".to_string());
                    error.set(Some(format!(
                        "RPC request failed with status {}: {}",
                        status, err_msg
                    )));
                }
            }
            Err(e) => {
                error.set(Some(format!("RPC request failed: {}", e)));
            }
        }

    });
}

// *** YOU NEED TO IMPLEMENT THIS FUNCTION BASED ON YOUR RPC'S JSON RESPONSE FORMAT ***
fn extract_accounts_data(json: &serde_json::Value) -> Vec<AccountInfo> {
    let mut accounts = Vec::new();

    // The 'result' field for getProgramAccounts is an array of objects
    // like { "pubkey": "...", "account": { ... } }
    if let Some(result_array) = json["result"].as_array() {
        for item in result_array {
            // Each item should have a "pubkey"
            if let Some(pubkey_value) = item["pubkey"].as_str() {
                let account_details = &item["account"];
                // And an "account" object containing "lamports" and "executable"
                if let Some(lamports) = account_details["lamports"].as_u64() {
                    let executable = account_details["executable"].as_bool().unwrap_or_else(|| {
                        eprintln!(
                            "Warning: Could not extract executable status for pubkey {}. Defaulting to false.",
                            pubkey_value
                        );
                        false
                    });
                    let account_data_json = &account_details["data"];
                    let account_data_str = serde_json::to_string(account_data_json).unwrap_or_else(|_| {
                        eprintln!(
                            "Warning: Could not serialize account data for pubkey {}. Defaulting to empty string.",
                            pubkey_value
                        );
                        String::new()
                    });

                    accounts.push(AccountInfo {
                        pubkey: pubkey_value.to_string(),
                        lamports: lamports,
                        executable: executable,
                        account_data: account_data_str,
                    });
                } else {
                    // Print a warning if lamports data is missing or wrong type for an account
                    eprintln!(
                        "Warning: Could not extract lamports from account data for pubkey {}. Skipping account.",
                        pubkey_value
                    );
                    // Optionally, you could still add the account with lamports set to 0
                    // accounts.push(AccountInfo {
                    //     pubkey: pubkey_value.to_string(),
                    //     lamports: 0,
                    //     executable: false, // Or parse executable separately if desired
                    // });
                }
            } else {
                // Print a warning if pubkey is missing or wrong type for an item
                eprintln!("Warning: Could not extract pubkey from item: {:?}. Skipping item.", item);
            }
        }
    } else {
        // Print an error if the "result" field is missing or not an array
        eprintln!(
            "Error: RPC response 'result' is missing or not an array or an error occurred: {:?}",
            json
        );
        // You might want to check for a top-level "error" field in the JSON here
        if let Some(rpc_error) = json["error"].as_object() {
            eprintln!("RPC Error: {:?}", rpc_error);
            // You could also try to set the Dioxus error signal here if needed
        }
    }

    accounts // Return the vector of parsed AccountInfo
}