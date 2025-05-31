use std::collections::HashMap;

use reqwest::{Error, Response};

pub struct NodeFecth {
    url: String,
}

impl NodeFecth {
    pub fn new(url: String) -> Self {
        NodeFecth { url }
    }
    pub async fn fecth_accounts(
        &self,
        search_request: HashMap<String, String>,
    ) -> Result<Response, Error> {
        reqwest::Client::new()
            .post(format!("{}/accounts", self.url))
            .header("Content-Type", "application/json")
            .json(&search_request)
            .send()
            .await
    }
}
