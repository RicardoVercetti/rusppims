use axum::{Json, extract::State};
use sqlx::{Pool, Sqlite};
use serde::{Serialize, Deserialize};

use crate::{
    store::CustomerInfo,
    utils,
    database::db::get_customer_by_mobile_number, // adjust module path if needed
};

// request handler
pub async fn check_customer_status_handler(
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<CheckCustomerStatusRequest>,
) -> Json<CheckCustomerResponse> {

    utils::print_req_res(&payload, "req");

    let mobile_number = &payload
        .data
        .check_reg_status
        .mobile_number;

    match get_customer_by_mobile_number(&pool, mobile_number).await {
        Ok(Some(customer)) => {
            let res = CheckCustomerResponse::from_customer("000", &customer);
            utils::print_req_res(&res, "res");
            Json(res)
        }

        Ok(None) => {
            let res = CheckCustomerResponse::new("404".to_string(), "NA".to_string());
            utils::print_req_res(&res, "res");
            Json(res)
        }

        Err(err) => {
            eprintln!("DB error: {:?}", err);
            let res = CheckCustomerResponse::new("500".to_string(), "NA".to_string());
            utils::print_req_res(&res, "res");
            Json(res)
        }
    }
}

// DTOs

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckRegistrationStatus {
    #[serde(rename = "Ref_Id")]
    pub ref_id: String,
    
    #[serde(rename = "Mobile_Number")]
    pub mobile_number: String,
    
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Date_Of_Birth")]
    pub date_of_birth: String,
     
    #[serde(rename = "Ovid_Type")]
    pub ovid_type: String,

    #[serde(rename = "Ovid_Value")]
    pub ovid_value: String,

    #[serde(rename = "System_Id")]
    pub system_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckCustomerStatusRequestData {
    #[serde(rename = "Check_Reg_Status")]
    pub check_reg_status: CheckRegistrationStatus,

    #[serde(rename = "Username")]
    pub username: String,

    #[serde(rename = "Password")]
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Risk {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckCustomerStatusRequest {
    #[serde(rename = "Data")]
    pub data: CheckCustomerStatusRequestData,

    #[serde(rename = "Risk")]
    pub risk: Risk

}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckCustomerStatusResponseData {
    #[serde(rename = "Resp_Code")]
    pub resp_code: String,
    
    #[serde(rename = "Unique_Id")]
    pub unique_id: String,

    #[serde(rename = "KYC_Flag")]
    pub kyc_flag: String,

    #[serde(rename = "KYC_Updated_Channel")]
    pub kyc_updated_channel: String,

    #[serde(rename = "KYC_Updated_On")]
    pub kyc_updated_on: String,

    #[serde(rename = "Cif_Id")]
    pub cif_id: String,

    #[serde(rename = "Remaining_Avail_Limit")]
    pub remaining_avail_limit: String,

    #[serde(rename = "Utilized_Bal")]
    pub utilized_bal: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckCustomerResponse {
    #[serde(rename = "Data")]
    data: CheckCustomerStatusResponseData,

    #[serde(rename = "Risk")]
    risk: Risk,

    #[serde(rename = "Links")]
    links: Links,

    #[serde(rename = "Meta")]
    meta: Meta

}

impl CheckCustomerResponse {
    fn from_customer(resp_code: &str, customer: &CustomerInfo) -> Self {
        CheckCustomerResponse {
            data: CheckCustomerStatusResponseData {
                resp_code: resp_code.to_string(),
                unique_id: customer.unique_id.clone(),
                kyc_flag: customer.kyc_flag.clone(),
                kyc_updated_channel: customer.kyc_updated_channel.clone(),
                kyc_updated_on: customer.kyc_updated_on.clone().unwrap_or_else(|| "NA".to_string()),
                cif_id: customer.cif_id.clone().unwrap_or_else(|| "NA".to_string()),
                remaining_avail_limit: format!("{:.2}", 0.0),
                utilized_bal: format!("{:.2}", customer.consumed),
            },
            risk: Risk {},
            links: Links {},
            meta: Meta {},
        }
    }

    fn new(resp_code: String, unique_id: String) -> Self {
        CheckCustomerResponse {
            data: CheckCustomerStatusResponseData {
                resp_code,
                unique_id,
                kyc_flag: "NA".to_string(),
                kyc_updated_channel: "NA".to_string(),
                kyc_updated_on: "NA".to_string(),
                cif_id: "NA".to_string(),
                remaining_avail_limit: "0.00".to_string(),
                utilized_bal: "0.00".to_string(),
            },
            risk: Risk {},
            links: Links {},
            meta: Meta {},
        }
    }
}
