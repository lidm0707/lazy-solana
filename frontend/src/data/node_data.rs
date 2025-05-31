use std::collections::HashMap;

use reqwest::{Error, Response};

use crate::fecth::node_fecth::NodeFecth;

//http://127.0.0.1:3000/

pub trait IntoDataNodes {
    async fn get_accounts(&self) -> Result<Response, Error>;
}

pub struct DataNode {
    search_request: String,
}

impl DataNode {
    pub fn new(search_request: String) -> Self {
        DataNode { search_request }
    }
}

impl IntoDataNodes for DataNode {
    async fn get_accounts(&self) -> Result<Response, Error> {
        let node_fetch = NodeFecth::new("http://127.0.0.1:3000".to_owned());
        let mut json: HashMap<String, String> = HashMap::new();
        json.insert("addr".to_owned(), self.search_request.to_owned());

        node_fetch.fecth_accounts(json).await
    }
}
