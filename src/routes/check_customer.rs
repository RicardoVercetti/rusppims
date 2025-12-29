use std::sync::Arc;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{store::CustomerInfo, utils::{self, find_by_mobile_number}};

// request handler
pub async fn check_customer_status_handler(
    State(_state): State<Arc<RwLock<Vec<CustomerInfo>>>>,
    Json(payload): Json<CheckCustomerStatusRequest>,
) -> Json<CheckCustomerResponse> {
    utils::print_req_res(&payload, "req");

    let customer_mob_req = &payload.data.check_reg_status.mobile_number;

    let customer_infos = _state.read().await;

    let found = find_by_mobile_number(&customer_mob_req, &customer_infos);

    match found {
        Some(c) => {
            let res = CheckCustomerResponse::new("000".to_string(), c.unique_id.to_string());
            utils::print_req_res(&res, "res");
            return Json(res);
        },

        None => {
            let res = CheckCustomerResponse::new("500".to_string(), "NA".to_string());
            utils::print_req_res(&res, "res");
            return Json(res);
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
    fn new(resp_code: String, unique_id: String) -> Self {
        CheckCustomerResponse { data: CheckCustomerStatusResponseData {
            resp_code: resp_code,
            unique_id: unique_id,
            kyc_flag: "NA".to_string(),
            kyc_updated_channel: "NA".to_string(),
            kyc_updated_on: "NA".to_string(),
            cif_id: "NA".to_string(),
            remaining_avail_limit: "0.00".to_string(),
            utilized_bal: "0.00".to_string()
        }, risk: Risk {  }, links: Links {  }, meta: Meta {  } }
    }
}