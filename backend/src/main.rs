use axum::{Json, Router, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize, Deserialize)]
struct AccountInfo {
    pubkey: String,
    lamports: u64,
}
#[derive(Serialize, Deserialize)]
pub struct PogramAddess {
    pub addr: String,
}

async fn get_accounts(Json(body): Json<PogramAddess>) -> (StatusCode, Json<Vec<AccountInfo>>) {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url);
    //"kJpCEdgKBL1T4N5zjdoe5bGn8kNFMwmX6bKmdMjSXen"
    let program_id = Pubkey::from_str(&body.addr).unwrap();

    let accounts: Vec<AccountInfo> = client
        .get_program_accounts(&program_id)
        .unwrap_or_default()
        .into_iter()
        .map(|(pubkey, account)| AccountInfo {
            pubkey: pubkey.to_string(),
            lamports: account.lamports,
        })
        .collect();

    (StatusCode::OK, Json(accounts))
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/accounts", post(get_accounts))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
