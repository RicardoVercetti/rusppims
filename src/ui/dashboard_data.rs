use serde::{Deserialize, Serialize};


use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::{Pool, Sqlite};

use crate::utils::print_req_res;
use crate::database::db::fetch_dashboard_data;

pub async fn handle_dashboard(
    State(pool): State<Pool<Sqlite>>,
) -> (StatusCode, Json<DashboardResponse>) {

    match fetch_dashboard_data(&pool).await {
        Ok(data) => {
            let res = DashboardResponse {
                status: "00".to_string(),
                message: "Dashboard data fetched successfully".to_string(),
                data,
            };

            print_req_res(&res, "res");
            (StatusCode::OK, Json(res))
        }

        Err(err) => {
            eprintln!("Dashboard error: {:?}", err);

            let res = DashboardResponse {
                status: "99".to_string(),
                message: "Failed to fetch dashboard data".to_string(),
                data: DashboardData {
                    transactions_today: 0,
                    total_customers: 0,
                    kyc_y1: 0,
                    kyc_y2: 0,
                    kyc_y3: 0,
                },
            };

            (StatusCode::INTERNAL_SERVER_ERROR, Json(res))
        }
    }
}

// DTOs
#[derive(Serialize, Deserialize, Debug)]
pub struct DashboardData {
    pub transactions_today: i64,
    pub total_customers: i64,
    pub kyc_y1: i64,
    pub kyc_y2: i64,
    pub kyc_y3: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DashboardResponse {
    pub status: String,
    pub message: String,
    pub data: DashboardData,
}
