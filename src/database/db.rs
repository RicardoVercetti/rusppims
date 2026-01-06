use sqlx::{Pool, Sqlite, SqlitePool, sqlite::SqliteQueryResult};

use crate::store::CustomerInfo;

pub async fn init_db() -> SqlitePool {
    // let base = std::env::current_dir().unwrap().join("data.db");
    let full_path = "sqlite://ppims_db.sqlite";

    println!("path_str: {}", full_path);
    SqlitePool::connect(full_path)      // this results in a single connection
        .await
        .expect("Failed to connect to SQLite")
}

pub async fn insert_customer(pool: &Pool<Sqlite>, customer: &CustomerInfo) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO customer_profile (
            unique_id,
            customer_name,
            maiden_name,
            mobile_number,
            date_of_birth,
            account_number,
            account_status,
            card_number,
            card_exp_date,
            card_status,
            kyc_flag,
            kyc_updated_channel,
            kyc_updated_on,
            ovid_value,
            ovid_type,
            cif_id,
            consumed
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&customer.unique_id)
    .bind(&customer.customer_name)
    .bind(&customer.maiden_name)
    .bind(&customer.mobile_number)
    .bind(&customer.date_of_birth)
    .bind(&customer.account_number)
    .bind(&customer.account_status)
    .bind(&customer.card_number)
    .bind(&customer.card_exp_date)
    .bind(&customer.card_status)
    .bind(&customer.kyc_flag)
    .bind(&customer.kyc_updated_channel)
    .bind(&customer.kyc_updated_on)
    .bind(&customer.ovid_value)
    .bind(&customer.ovid_type)
    .bind(&customer.cif_id)
    .bind(customer.consumed)
    .execute(pool)
    .await
}
