use anyhow::Result;
use dioxus_logger::tracing;
use serde_json::json;

use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct ResultData {
    pub result: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
pub struct Entry {
    pub account: Account,
    pub pubkey: String,
}

#[derive(Debug)]
pub struct RentEpoch(pub String);

impl<'de> Deserialize<'de> for RentEpoch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::String(s) => Ok(RentEpoch(s)),
            serde_json::Value::Number(n) => Ok(RentEpoch(n.to_string())),
            _ => Err(serde::de::Error::custom(
                "Expected string or number for rentEpoch",
            )),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub data: Vec<String>,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: RentEpoch, // Changed to String to handle large values
    pub space: u64,
}

pub async fn search_account(
    rpc_url: String,
    program_address: String,
    target_discriminator: Vec<u8>,
) -> Result<ResultData> {
    use anyhow::Context;

    // Convert the target discriminator to Base58
    let target_discriminator_base58 = bs58::encode(&target_discriminator).into_string();
    println!(
        "Target discriminator in Base58: {:?}",
        target_discriminator_base58
    );

    // Create the JSON RPC request body
    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getProgramAccounts",
        "params": [
            program_address,
            {
                "encoding": "base64",
                "filters": [
                    {
                        "memcmp": {
                            "offset": 0,
                            "bytes": target_discriminator_base58
                        }
                    }
                ]
            }
        ]
    });

    // Send the HTTP POST request
    let client = reqwest::Client::new();
    let response = client
        .post(&rpc_url)
        .json(&request_body)
        .send()
        .await
        .context("Failed to send HTTP request")?;

    // Ensure the HTTP response was successful
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "HTTP request failed with status: {}",
            response.status()
        ));
    }

    // Parse the response body into the ResultData struct
    let response_body = response
        .text()
        .await
        .context("Failed to read response body")?;
    let result_data: ResultData = serde_json::from_str(&response_body)
        .context("Failed to parse response JSON into ResultData")?;
    tracing::debug!("result_data: {:?}", result_data);
    // Return the parsed data
    Ok(result_data)
}
