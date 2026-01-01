use axum::{
    Json,
    http::StatusCode
};
use serde::{Deserialize, Serialize};

use crate::utils::print_req_res;


#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    username: String,
    password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    status: String, 
    message: String,
}

pub async fn handle_login(Json(login_payload): Json<LoginRequest>) -> (StatusCode, Json<LoginResponse>) {
    print_req_res(&login_payload, "req");

    let res = {
        if login_payload.username == "admin" && login_payload.password == "admin@123" {
            (StatusCode::OK, LoginResponse { status: "00".to_string(), message: "Login successful".to_string() })
        } else {
            (StatusCode::UNAUTHORIZED, LoginResponse { status: "10".to_string(), message: "invalid credentials".to_string() })
        }
    };

    print_req_res(&res.1, "res");
    (res.0, Json(res.1))
}