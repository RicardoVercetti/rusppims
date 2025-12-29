use std::sync::Arc;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{store::{CustomerInfo, save_to_file}, utils::{find_n_get_mut_customer_info, print_req_res}};
pub async fn handle_update_customer_limit(
    State(_state): State<Arc<RwLock<Vec<CustomerInfo>>>>,
    Json(payload): Json<UpdateCustomerLimitRequest>
) -> Json<UpdateCustomerLimitResponse> {
    
    print_req_res(&payload, "Req");

    // update the 'consumed' of CustomerInfo to consumed + amount(from request)
    // assumptions:
    // we don't check the limit here, its checked at check customer limit call
    // always Debit, credit will be added later
    // assume that the transaction status from the request is always 'S'(for success), I don't know the scenario where it would be 'F' and what to do then.

    let mut locked_all_customers = _state.write().await;

    let customer_info = find_n_get_mut_customer_info(&mut locked_all_customers, &payload.data.update_customer.unique_id);

    let res = match customer_info {
        Some(cus) => {
            // update the limit

            let amount_to_consume: Result<f32, std::num::ParseFloatError> = payload.data.update_customer.amount.parse::<f32>();
            match amount_to_consume {     // parsing into float 32 failed
                Err(err) => {
                    println!("{}", err);
                    UpdateCustomerLimitResponse::new("400".to_string(), cus.unique_id.to_string())
                },
                Ok(amt) => {
                    // here update the consumed amount
                    cus.consumed += amt;
                    let unique_id = cus.unique_id.to_string();
                    drop(locked_all_customers);
                    let read_only_all_customers = _state.read().await;

                    let save_status = save_to_file(&read_only_all_customers).await;
                    if let Err(err) = save_status {
                        print!("saving to file failed: {}", err);
                    };

                    UpdateCustomerLimitResponse::new("200".to_string(), unique_id)
                }
            }
        },
        None => UpdateCustomerLimitResponse::new("500".to_string(), "NA".to_string()),
    };

    print_req_res(&res, "Res");
    Json(res)
}

// DTOs
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCustomerLimitRequest {
    #[serde(rename="Data")]
    pub data: UpdateCustomerLimitRequestData,

    #[serde(rename="Risk")]
    pub risk: Risk,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCustomerLimitRequestData {
    #[serde(rename="Update_Limit")]
    pub update_customer: UpdateLimit,

    #[serde(rename="Username")]
    pub username: String,

    #[serde(rename="Password")]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateLimit {
    #[serde(rename="Ref_Id")]
    pub ref_id: String,

    #[serde(rename="Unique_Id")]
    pub unique_id: String,

    #[serde(rename="Amount")]
    pub amount: String,
    
    #[serde(rename="Tran_Status")]
    pub tran_status: String,

    #[serde(rename="Tran_Type")]
    pub tran_type: String,

    #[serde(rename="Avail_Bal")]
    pub avail_bal: String,

    #[serde(rename="System_Id")]
    pub system_id: String,

    #[serde(rename="Enquiry_Ref_Id")]
    pub enquiry_ref_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Risk {}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCustomerLimitResponse {
    #[serde(rename="Data")]
    pub data: UpdateCustomerLimitResponseData,
    #[serde(rename="Risk")]
    pub risk: Risk,
    #[serde(rename="Links")]
    pub links: Links,
    #[serde(rename="Meta")]
    pub meta: Meta
}

impl UpdateCustomerLimitResponse {
    fn new(resp_code: String, unique_id: String) -> Self {
        UpdateCustomerLimitResponse {
            data: UpdateCustomerLimitResponseData { resp_code: resp_code, unique_id: unique_id, old_unique_id: "NA".to_string() },
            risk: Risk {  },
            links: Links {  },
            meta: Meta {  }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCustomerLimitResponseData {
    pub resp_code: String,
    pub unique_id: String,
    pub old_unique_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {}