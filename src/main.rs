mod routes;
mod server;
mod store;
mod utils;
mod core;

use server::start_server;

#[tokio::main]
async fn main() {
    start_server().await;
}
