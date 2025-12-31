use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {
    // let base = std::env::current_dir().unwrap().join("data.db");
    let full_path = "sqlite://data.db";

    println!("path_str: {}", full_path);
    SqlitePool::connect(full_path)      // this results in a single connection
        .await
        .expect("Failed to connect to SQLite")
}

// create default tables if doesn't exist
