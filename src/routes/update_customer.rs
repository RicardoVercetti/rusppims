use crate::store::{CustomerInfo, save_to_file};
use crate::utils::{self, find_n_get_mut_customer_info};
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCustomerRequest {
    #[serde(rename = "Data")]
    data: UpdateCustomerData,
    #[serde(rename = "Risk")]
    risk: Risk,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdateCustomerData {
    #[serde(rename = "Update_Customer")]
    update_customer: UpdateCustomer,

    #[serde(rename = "Username")]
    username: String,

    #[serde(rename = "Password")]
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCustomer {
    #[serde(rename = "Ref_Id")]
    pub ref_id: String,

    #[serde(rename = "Unique_Id")]
    pub unique_id: String,

    #[serde(rename = "Mobile_Number")]
    pub mobile_number: String,

    #[serde(rename = "Customer_Name")]
    pub customer_name: String,

    #[serde(rename = "Date_Of_Birth")]
    pub date_of_birth: String,

    #[serde(rename = "Email_Id")]
    pub email_id: String,

    #[serde(rename = "Account_Number")]
    pub account_number: String,

    #[serde(rename = "Account_Status")]
    pub account_status: String,

    #[serde(rename = "Card_Number")]
    pub card_number: String,

    #[serde(rename = "Card_Status")]
    pub card_status: String,

    #[serde(rename = "KYC_Flag")]
    pub kyc_flag: String,

    #[serde(rename = "KYC_Updated_Channel")]
    pub kyc_update_channel: String,

    #[serde(rename = "KYC_Updated_On")]
    pub kyc_update_on: String,

    #[serde(rename = "Maiden_Name")]
    pub maiden_name: String,

    #[serde(rename = "Ovid_Type")]
    pub ovid_type: String,

    #[serde(rename = "Ovid_Value")]
    pub ovid_value: String,

    #[serde(rename = "System_Id")]
    pub system_id: String,

    #[serde(rename = "Cif_Id")]
    pub cif_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Risk {}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCustomerResponse {
    #[serde(rename = "Data")]
    data: UpdateCustomerResponseData,

    #[serde(rename = "Risk")]
    risk: Risk,

    #[serde(rename = "Links")]
    link: Links,

    #[serde(rename = "Meta")]
    meta: Meta,
}

impl UpdateCustomerResponse {
    fn new(resp_code: &str, unique_id: &str, old_unique_id: &str) -> Self {
        UpdateCustomerResponse {
            data: UpdateCustomerResponseData {
                resp_code: resp_code.to_string(),
                unique_id: unique_id.to_string(),
                old_unique_id: old_unique_id.to_string(),
            },
            risk: Risk {},
            link: Links {},
            meta: Meta {},
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdateCustomerResponseData {
    #[serde(rename = "Resp_Code")]
    resp_code: String,

    #[serde(rename = "Unique_Id")]
    unique_id: String,

    #[serde(rename = "Old_Unique_Id")]
    old_unique_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Links {}

#[derive(Serialize, Deserialize, Debug)]
struct Meta {}

pub async fn update_customer_handler(
    State(state): State<Arc<RwLock<Vec<CustomerInfo>>>>,
    Json(payload): Json<UpdateCustomerRequest>,
) -> Json<UpdateCustomerResponse> {
    // now start handling stuff
    utils::print_req_res(&payload, "req");
    let request_customer_data = &payload.data.update_customer;

    // find the customer from the list and see if you can update data for that customer
    let mut customer_list = state.write().await;
    let customer_in_question = {
        let found_customer = find_n_get_mut_customer_info(
            &mut *customer_list,
            &payload.data.update_customer.unique_id,
        );
        found_customer
    };

    let res: UpdateCustomerResponse;
    match customer_in_question {
        Some(customer) => {
            // update customer
            customer.update_customer(request_customer_data);
            println!("customer data updated successfully...");
            res = UpdateCustomerResponse::new("000", &request_customer_data.unique_id, "NA");

            // write the updated data and drop the lock
            let result = save_to_file(&*customer_list).await;
            if let Err(e) = result {
                println!("Error while saving file: {:?}", e);
            }
        }
        None => {
            // No customer found to update
            println!(
                "No customer found for ID: {}",
                &request_customer_data.unique_id
            );
            res = UpdateCustomerResponse::new("400", &request_customer_data.unique_id, "NA");
        }
    }

    utils::print_req_res(&res, "res");
    Json(res)
}
