use std::sync::Arc;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{core::{calc::calculate_cum_balance, kyc::KycTypes}, store::CustomerInfo, utils::{find_by_unique_id, print_req_res}};

pub async fn handle_check_customer_limit(
    State(state): State<Arc<RwLock<Vec<CustomerInfo>>>>,
    Json(payload): Json<CheckCustomerLimitRequest>
) -> Json<CheckCustomerLimitResponse> {
    print_req_res(&payload, "req");


    // check if amount from request + Avail balance <= Max limit for the KYC
    // if 'yes', the response is '000' with allow_customer 'T'
    // else the response is gonna be '000' with allow_customer 'F'
    // if customer info not found, '500' with allow_customer 'F'

    let customer_read = state.read().await;

    let customer_info = find_by_unique_id(&payload.data.check_limit.unique_id, &customer_read);

    let res: CheckCustomerLimitResponse = match customer_info {
        Some(cus) => {

            // if limit satisfies
            let is_okay = check_the_limit(cus, &payload);
            
            match is_okay {
                Ok(v) => {
                    if v {
                        let r = CheckCustomerLimitResponse::new(
                            "000".to_string(),
                            cus.unique_id.to_string(),
                            "T".to_string(),
                            calculate_cum_balance(&cus, &payload.data.check_limit.amount),
                            "0.00".to_string(), // avail limit
                            cus.cif_id.as_deref().unwrap_or("NA").to_string(),
                        );
                        r
                    } else {
                        let r = CheckCustomerLimitResponse::new(
                            "000".to_string(),
                            cus.unique_id.to_string(),
                            "F".to_string(),
                            format!("{:.02}", &cus.consumed),       // for failed, cum balance == consumed
                            "0.00".to_string(), // avail limit
                            cus.cif_id.as_deref().unwrap_or("NA").to_string(),
                        );
                        println!("not enough balance");
                        r
                    }
                },
                Err(s) => {
                    println!("Error occurred!");
                    println!("{}", s);
                    let r = CheckCustomerLimitResponse::new(
                            "000".to_string(),
                            cus.unique_id.to_string(),
                            "F".to_string(),
                            "0.00".to_string(),
                            "0.00".to_string(), // avail limit
                            cus.cif_id.as_deref().unwrap_or("NA").to_string(),
                        );
                    r
                }
            }
            },
        None => {
            println!("customer not found!");
            let res = CheckCustomerLimitResponse::new(
                "500".to_string(),
                "NA".to_string(),
                "F".to_string(),
                "NA".to_string(),
                "NA".to_string(),
                "NA".to_string()
            );
            res
        },
    };

    print_req_res(&res, "res");
    Json(res)
}

fn check_the_limit(cus: &CustomerInfo, req: &CheckCustomerLimitRequest) -> Result<bool, String> {       // assume its debit for now, todo: handle for credit
    let amount_from_req: f32 = req.data.check_limit.amount
                .parse::<f32>()
                .map_err(|e| format!("Invalid amount: {}", e))?;

    let customer_kyc = KycTypes::from_code(&cus.kyc_flag)?;
    if customer_kyc.is_under_limit(&cus.consumed, &amount_from_req) {
        return Ok(true)
    }

    Ok(false)
}

// DTOs

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckCustomerLimitRequest {
    #[serde(rename="Data")]
    pub data: CheckCustomerLimitRequestData,
    #[serde(rename="Risk")]
    pub risk: Risk
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckCustomerLimitRequestData {
    #[serde(rename="Check_Limit")]
    pub check_limit: CheckLimit,
    #[serde(rename="Username")]
    pub username: String,
    #[serde(rename="Password")]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckLimit {
    #[serde(rename="Ref_Id")]
    pub ref_id: String,
    #[serde(rename="Unique_Id")]
    pub unique_id: String,
    #[serde(rename="Account_Number")]
    pub account_number: String,
    #[serde(rename="Card_Number")]
    pub card_number: String,
    #[serde(rename="Amount")]
    pub amount: String,
    #[serde(rename="Tran_Type")]
    pub tran_type: String,
    #[serde(rename="Avail_Bal")]
    pub avail_bal: String,
    #[serde(rename="System_Id")]
    pub system_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Risk {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckCustomerLimitResponse {
    #[serde(rename="Data")]
    pub data: CheckCustomerLimitResponseData,
    #[serde(rename="Risk")]
    pub risk: Risk,
    #[serde(rename="Links")]
    pub links: Links,
    #[serde(rename="Meta")]
    pub meta: Meta
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckCustomerLimitResponseData {
    #[serde(rename="Resp_Code")]
    pub resp_code: String,
    #[serde(rename="Unique_Id")]
    pub unique_id: String,
    #[serde(rename="Allow_Customer")]
    pub allow_customer: String,
    #[serde(rename="Cumulative_Bal")]
    pub cumulative_bal: String,
    #[serde(rename="Avail_Amount_Limit")]
    pub avail_amount_limit: String,
    #[serde(rename="Cif_Id")]
    pub cif_id: String,
    #[serde(rename="Old_Unique_Id")]
    pub old_unique_id: String,
}

impl CheckCustomerLimitResponse {
    fn new(resp_code: String, unique_id: String, allow_cus: String, cum_bal: String, avail_limit: String, cif_id: String) -> Self {
        CheckCustomerLimitResponse{
            data: CheckCustomerLimitResponseData {
                resp_code: resp_code,
                unique_id: unique_id,
                allow_customer: allow_cus,
                cumulative_bal: cum_bal,
                avail_amount_limit: avail_limit,
                cif_id: cif_id,
                old_unique_id: "NA".to_string()
            },
            risk: Risk {}, 
            links: Links {}, 
            meta: Meta {} 
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {}