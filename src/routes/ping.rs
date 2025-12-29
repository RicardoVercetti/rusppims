use crate::{store::CustomerInfo, utils};
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// simple pings

#[derive(Serialize, Deserialize, Debug)]
pub struct PingPostData {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct PingPostResponse {
    message: String,
}

pub async fn ping_get() -> String {
    "Rusppims here, howdy partner!".to_string()
}

pub async fn ping_post(
    State(state): State<Arc<RwLock<Vec<CustomerInfo>>>>,
    Json(payload): Json<PingPostData>,
) -> Json<PingPostResponse> {
    utils::print_req_res(&payload, "req");
    // you can do something with the data ya know!

    let data: tokio::sync::RwLockReadGuard<'_, Vec<CustomerInfo>> = state.read().await;

    println!("all customer info: {:#?}", data);

    Json(PingPostResponse {
        message: "Request received successfully".to_string(),
    })
}
