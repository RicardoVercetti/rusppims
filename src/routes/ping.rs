use crate::{database::db, utils};
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

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
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<PingPostData>,
) -> Json<PingPostResponse> {
    utils::print_req_res(&payload, "req");
    // you can do something with the data ya know!

    let data = db::get_all_customers(&pool).await;
    match data {
        Ok(customers) => println!("all customer info: {:#?}", customers),
        Err(err) => println!("error: {}", err),
    }

    

    Json(PingPostResponse {
        message: "Request received successfully".to_string(),
    })
}
