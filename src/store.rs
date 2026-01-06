use crate::routes::update_customer::UpdateCustomer;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::Error;
use tokio::{
    fs::{File, metadata, read_to_string},
    io::AsyncWriteExt,
};
use sqlx::FromRow;

use crate::routes::add_customer::AddCustomer;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct CustomerInfo {
    pub unique_id: String, // this is the ppid
    pub customer_name: String,
    pub maiden_name: Option<String>,
    pub mobile_number: String,
    pub date_of_birth: String,
    pub account_number: String,
    pub account_status: String,
    pub card_number: Option<String>,
    pub card_exp_date: Option<String>,
    pub card_status: Option<String>,
    pub kyc_flag: String,
    pub kyc_updated_channel: String,
    pub kyc_updated_on: Option<String>,
    pub ovid_value: Option<String>,
    pub ovid_type: Option<String>,
    pub cif_id: Option<String>,
    pub consumed: f32,
}

impl CustomerInfo {
    pub fn new(add_customer: &AddCustomer, ppid: &String) -> CustomerInfo {
        CustomerInfo {
            unique_id: ppid.clone(),
            customer_name: add_customer.customer_name.clone(),
            maiden_name: add_customer.maiden_name.clone(),
            mobile_number: add_customer.mobile_number.clone(),
            date_of_birth: add_customer.date_of_birth.clone(),
            account_number: add_customer.account_number.clone(),
            account_status: add_customer.account_status.clone(),
            card_number: add_customer.card_number.clone(),
            card_exp_date: add_customer.card_exp_date.clone(),
            card_status: add_customer.card_status.clone(),
            kyc_flag: add_customer.kyc_flag.clone(),
            kyc_updated_channel: add_customer.kyc_updated_channel.clone(),
            kyc_updated_on: add_customer.kyc_updated_on.clone(),
            ovid_value: add_customer.ovid_value.clone(),
            ovid_type: add_customer.ovid_type.clone(),
            cif_id: add_customer.cif_id.clone(),
            consumed: 0.00f32,
        }
    }

    pub fn update_customer(&mut self, update_customer: &UpdateCustomer) {
        self.unique_id = update_customer.unique_id.clone();
        self.mobile_number = update_customer.mobile_number.clone();
        self.customer_name = update_customer.customer_name.clone();
        self.date_of_birth = update_customer.date_of_birth.clone();
        // self.email_id = update_customer.email_id.clone();
        self.account_number = update_customer.account_number.clone();
        self.account_status = update_customer.account_status.clone();
        self.card_number = Some(update_customer.card_number.clone());
        self.card_status = Some(update_customer.card_status.clone());
        self.kyc_flag = update_customer.kyc_flag.clone();
        self.kyc_updated_on = Some(update_customer.kyc_update_on.clone());
        self.maiden_name = Some(update_customer.maiden_name.clone());
        self.ovid_type = Some(update_customer.ovid_type.clone());
        self.ovid_value = Some(update_customer.ovid_value.clone());
        self.cif_id = Some(update_customer.cif_id.clone());
    }
}



// write to file
pub async fn save_to_file(customers: &Vec<CustomerInfo>) -> Result<(), Box<dyn std::error::Error>> {
    // Convert your vector to pretty json
    let json: String = serde_json::to_string_pretty(customers)?;

    // Save to file asynchronously
    let mut file = File::create("customers.json").await?; // wipes the file on creation
    file.write_all(json.as_bytes()).await?;

    Ok(())
}

// load/create file
pub async fn load_or_create_file() -> Result<String, Error> {
    let filename = "customers.json";
    if metadata(filename).await.is_ok() {
        let content: Result<String, Error> = read_to_string(filename).await;
        return content;
    }

    // file doesn't exist, create one
    println!("customers.json is not found, initiating one...");
    let mut file = File::create(filename).await?;
    file.write_all(b"[]").await?;

    return Ok("[]".to_string());
}

// deserialize customer info from the customer.json string
pub fn deserialize_from_json_string(str: &str) -> Result<Vec<CustomerInfo>, serde_json::Error> {
    let customers_info: Vec<CustomerInfo> = serde_json::from_str(&str)?;
    Ok(customers_info)
}

// generate 15 digit ppid
pub fn generate_ppid() -> String {
    let val: i64 = rand::thread_rng().gen_range(0..=999_999_999_999_999);
    let padded_string = format!("{:0>15}", val);
    padded_string
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tokio::fs;

    #[tokio::test]
    async fn test_load_or_create_file() {
        // Remove the file if it exists (cleanup before test)
        if Path::new("customers.json").exists() {
            fs::remove_file("customers.json").await.unwrap();
        }

        // Call your function
        let result = load_or_create_file().await.unwrap();

        // Should return "{}"
        assert_eq!(result, "{}");

        // File should now exist
        assert!(Path::new("customers.json").exists());

        // Cleanup after test
        fs::remove_file("customers.json").await.unwrap();
    }

    #[tokio::test]
    async fn test_deserialize_json_string() {
        let json = r#"
    [
        {
            "unique_id": "123",
            "maiden_name": "Doe",
            "mobile_number": "555111222",
            "date_of_birth": "1990-01-01",
            "account_number": "111222",
            "account_status": "ACTIVE",
            "card_number": "4444555566667777",
            "card_exp_date": "12/30",
            "card_status": "ACTIVE",
            "kyc_flag": "Y",
            "kyc_updated_channel": "ONLINE",
            "kyc_updated_on": "2024-01-02",
            "ovid_value": "something",
            "ovid_type": "type",
            "cif_id": "99887"
        }
    ]
    "#;

        let data = deserialize_from_json_string(json).unwrap();
        println!("{:#?}", data);
    }
}
