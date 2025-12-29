use crate::{store::{CustomerInfo, generate_ppid, save_to_file}, utils::{ is_customer_exits_by_mobile_number, print_req_res}};
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// add customer
#[derive(Serialize, Deserialize, Debug)]
pub struct AddCustomer {
    #[serde(rename = "Ref_Id")]
    pub ref_id: String,

    #[serde(rename = "Unique_Id")]
    pub unique_id: Option<String>,

    #[serde(rename = "Customer_Id")]
    pub customer_id: String,

    #[serde(rename = "Customer_Name")]
    pub customer_name: String,

    #[serde(rename = "Maiden_Name")]
    pub maiden_name: Option<String>,

    #[serde(rename = "Mobile_Number")]
    pub mobile_number: String,

    #[serde(rename = "Date_Of_Birth")]
    pub date_of_birth: String,

    #[serde(rename = "Email_Id")]
    pub email_id: Option<String>,

    #[serde(rename = "Address_Line1")]
    pub address_line1: Option<String>,

    #[serde(rename = "Address_Line2")]
    pub address_line2: Option<String>,

    #[serde(rename = "City")]
    pub city: Option<String>,

    #[serde(rename = "State")]
    pub state: Option<String>,

    #[serde(rename = "Pincode")]
    pub pincode: Option<String>,

    #[serde(rename = "Account_Number")]
    pub account_number: String,

    #[serde(rename = "Account_Status")]
    pub account_status: String,

    #[serde(rename = "Card_Number")]
    pub card_number: Option<String>,

    #[serde(rename = "Card_Exp_date")]
    pub card_exp_date: Option<String>,

    #[serde(rename = "Card_Status")]
    pub card_status: Option<String>,

    #[serde(rename = "KYC_Flag")]
    pub kyc_flag: String,

    #[serde(rename = "KYC_Updated_Channel")]
    pub kyc_updated_channel: String,

    #[serde(rename = "KYC_Updated_On")]
    pub kyc_updated_on: Option<String>,

    #[serde(rename = "System_Id")]
    pub system_id: String,

    #[serde(rename = "Ovid_Value")]
    pub ovid_value: Option<String>,

    #[serde(rename = "Ovid_Type")]
    pub ovid_type: Option<String>,

    #[serde(rename = "Cif_Id")]
    pub cif_id: Option<String>,

    #[serde(rename = "Customer_Type")]
    pub customer_type: String,

    #[serde(rename = "Ppi_Type")]
    pub ppi_type: Option<String>,

    #[serde(rename = "NRI_Flag")]
    pub nri_flag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename = "Add_Customer")]
    add_customer: AddCustomer,

    #[serde(rename = "Username")]
    username: String,

    #[serde(rename = "Password")]
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Risk {}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCustomerRequst {
    #[serde(rename = "Data")]
    data: Data,

    #[serde(rename = "Risk")]
    risk: Risk,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCustomerResponse {
    #[serde(rename = "Data")]
    data: AddCustomerResponseData,

    #[serde(rename = "Risk")]
    risk: AddCustomerResponseRisk,

    #[serde(rename = "Links")]
    links: AddCustomerResponseLinks,

    #[serde(rename = "Meta")]
    meta: AddCustomerResponseMeta,
}

impl AddCustomerResponse {
    fn new(resp: &str, id: &str, flag: &str, chan: &str, up: &str) -> Self {
        AddCustomerResponse {
            data: AddCustomerResponseData {
                response_code: resp.to_string(),
                unique_id: id.to_string(),
                kyc_flag: flag.to_string(),
                kyc_updated_channel: chan.to_string(),
                kyc_updated_on: up.to_string(),
            },
            risk: AddCustomerResponseRisk {},
            links: AddCustomerResponseLinks {},
            meta: AddCustomerResponseMeta {},
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCustomerResponseData {
    #[serde(rename = "Resp_Code")]
    response_code: String,

    #[serde(rename = "Unique_Id")]
    unique_id: String,

    #[serde(rename = "KYC_Flag")]
    kyc_flag: String,

    #[serde(rename = "KYC_Updated_Channel")]
    kyc_updated_channel: String,

    #[serde(rename = "KYC_Updated_On")]
    kyc_updated_on: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCustomerResponseRisk {}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCustomerResponseLinks {}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCustomerResponseMeta {}

pub async fn add_customer_handler(
    State(state): State<Arc<RwLock<Vec<CustomerInfo>>>>,
    Json(payload): Json<AddCustomerRequst>,
) -> Json<AddCustomerResponse> {
    // request should be successfully received
    print_req_res(&payload, "req");
    let in_data = &payload.data.add_customer;

    // get customer data and check id
    let customer_data = state.read().await;

    // dedupe check for if customer exists with the same mobile number
    let is_customer_exist = is_customer_exits_by_mobile_number(
        &payload.data.add_customer.mobile_number,
        &customer_data,
    );
    drop(customer_data);
    if !is_customer_exist {
        // must generate PPID and save the same
        let ppid = generate_ppid();
        println!("generated ppid: {}", ppid);

        // map the AddCustomer to CustomerInfo

        let customer_info_map = CustomerInfo::new(in_data, &ppid);
        let mut customers = state.write().await;

        // add to vec
        customers.push(customer_info_map);

        // async save to file
        let save_res = save_to_file(&customers).await;
        match save_res {
            Err(e) => println!("error occurred while saving the file: {}", e),
            _ => {}
        }

        let res = AddCustomerResponse::new(
            "000",
            &ppid,
            &in_data.kyc_flag,
            &in_data.kyc_updated_channel,
            option_alt(&in_data.kyc_updated_on),
        );
        println!("res: {:#?}", res);
        return Json(res);
    }

    let res = AddCustomerResponse::new(
        "400",
        option_alt(&in_data.unique_id),
        &in_data.kyc_flag,
        &in_data.kyc_updated_channel,
        option_alt(&in_data.kyc_updated_on),
    );
    print_req_res(&res, "res");
    Json(res)
}

fn option_alt(opt: &Option<String>) -> &str {
    match opt {
        Some(str) => str,
        None => "N/A",
    }
}
