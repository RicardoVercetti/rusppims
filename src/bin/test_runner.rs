use core::fmt;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct User {
    id: u32,
    name: String,
}

impl User {
    fn new(id: u32, name: String) -> Self {
        User { id, name }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}", self.id, self.name)
    }
}

fn main() {
    println!("Hi, this is a separate binary...");

    let u = User::new(2, "Alice".to_string());

    println!("{}", u);

    let formatted = serde_json::to_string_pretty(&u).unwrap();
    println!("{}", formatted);
}