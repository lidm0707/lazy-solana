use std::collections::HashMap;

use dioxus::prelude::macro_helpers::const_serialize::SerializeConst;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};

use crate::fecth::node_fecth::NodeFecth;

//http://127.0.0.1:3000/

pub trait IntoDataNodes {
    fn new(search_request: String) -> Self;
    async fn get_accounts(&self) -> Result<Response, Error>;
}
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct DataNode {
    search_request: String,
}

impl IntoDataNodes for DataNode {
    fn new(search_request: String) -> Self {
        DataNode { search_request }
    }
    async fn get_accounts(&self) -> Result<Response, Error> {
        let node_fetch = NodeFecth::new("http://127.0.0.1:3000".to_owned());
        let mut json: HashMap<String, String> = HashMap::new();
        json.insert("addr".to_owned(), self.search_request.to_owned());

        node_fetch.fecth_accounts(json).await
    }
}
