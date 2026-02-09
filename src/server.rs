use crate::database::db::init_db;
use crate::routes::add_customer::add_customer_handler;
use crate::routes::check_customer::check_customer_status_handler;
// use crate::routes::check_customer_kyc::handle_check_customer_kyc;
// use crate::routes::check_customer_limit::handle_check_customer_limit;
use crate::routes::ping::{ping_get, ping_post};
// use crate::routes::update_customer::update_customer_handler;
// use crate::routes::update_customer_limit::handle_update_customer_limit;
// use crate::store::{CustomerInfo, deserialize_from_json_string, load_or_create_file};
use crate::ui::login::handle_login;
use axum::{
    Router,
    routing::{
        get, 
        post
    },
};
use tower_http::cors::{CorsLayer, Any};
// use std::sync::Arc;
use tokio::{
    net::TcpListener, 
    // sync::RwLock
};

pub async fn start_server() {
    // load the customer data on startup
    let pool: sqlx::Pool<sqlx::Sqlite> = init_db().await;
    // let json_str = load_or_create_file().await.unwrap();
    // let customers: Vec<CustomerInfo> = deserialize_from_json_string(&json_str).unwrap();

    // Wrap in Arc<RwLock>
    // let shared_state: Arc<RwLock<Vec<CustomerInfo>>> = Arc::new(RwLock::new(customers));

    // cors
    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);

    let app: Router = Router::new()
        .route("/", get(ping_get).post(ping_post))      // here we can host the static UI content
        .route("/login", post(handle_login))
        .route(
            "/axis/non-dmz/api/PPIM/v1/add-customer",
            post(add_customer_handler),
        )
        // .route("/axis/non-dmz/api/PPIM/v1/check-customer-kyc", post(handle_check_customer_kyc))                // check customer kyc
        // .route("/axis/non-dmz/api/PPIM/v1/check-customer-limit", post(handle_check_customer_limit))              // check customer limit
        .route("/axis/non-dmz/api/PPIM/v1/customer-registration-status", post(check_customer_status_handler))      // check customer registration status
        // .route(
            // "/axis/non-dmz/api/PPIM/v1/update-customer",
            // post(update_customer_handler),
        // ) // update customer
        // .route("/axis/non-dmz/api/PPIM/v1/update-customer-limit", post(handle_update_customer_limit))             // update customer limit
        .with_state(pool.clone())
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let port: &str = "0.0.0.0:3000";
    let listener: TcpListener = tokio::net::TcpListener::bind(port).await.unwrap();

    let server = axum::serve(listener, app);
    println!("Running http server on :{port}");
    server.await.unwrap();
}
