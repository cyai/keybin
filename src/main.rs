mod core {
    pub mod commands {
        pub mod get;
    }
}

use core::commands::get::get;
use dotenv::dotenv;
use std::io;

#[tokio::main]
async fn main() {
    let mut secret_id = String::new();
    dotenv().ok();

    println!("Hello, world!");
    println!("Enter secret id: ");
    io::stdin().read_line(&mut secret_id).expect("Failed to read line");

    let secret_id = secret_id.trim();

    let result = get(&secret_id).await;
    println!("{:?}", result);

}
