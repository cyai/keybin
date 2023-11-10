mod core {
    pub mod commands {
        pub mod get;
        pub mod list;
        pub mod store;
        pub mod update;
    }
}

use core::commands::{get::get, list::list, store::store, update::update};
use dotenv::dotenv;
use std::io;
use serde_json::Value;

#[tokio::main]
async fn main(){
    let mut secret_id = String::new();
    let mut service = String::new();
    dotenv().ok();

    println!("Hello, world!");
    println!("Choose the service: ");
    io::stdin().read_line(&mut service).expect("Failed to read service from user");
    let service = service.trim();

    if service == "get" {
        println!("Enter secret id: ");
        io::stdin().read_line(&mut secret_id).expect("Failed to read line");

        let secret_id = secret_id.trim();

        let result = get(&secret_id).await;
        println!("{:?}", result);
    } else if service == "list" {

        let result = list(None, None, None, None, Some(10), None, None, None, Some(false)).await; 

        // println!("{:?}", result);
        
        if let Ok(json_result) = result{
            let result: Value = serde_json::from_str(&json_result).unwrap();
            // let items = result["result"]["items"].as_array().ok_or("No items found");
            if let Some(items) = result["result"]["items"].as_array(){
                for item in items{
                    let name = item["name"].as_str().unwrap_or("!! No name found !!");
                    let key_type = item["type"].as_str().unwrap_or("!! No type found !!");
                    let created = item["created_at"].as_str().unwrap_or("!! No created date found !!");
                    let state = item["current_version"]["state"].as_str().unwrap_or("!! No status found !!");

                    println!("Name: {} \nType: {} \nCreated: {} \nState: {}\n", name, key_type, created, state);
                    println!("-------------------------------------------------");
                }
            } else {
                println!("No items found");
            }
        } else {
            eprintln!("Failed to retrieve data from the service.");
        }

    } else if service == "store" {
        let mut secret = String::new();
        let mut name = String::new();

        println!("Enter the name: ");   
        io::stdin().read_line(&mut secret).expect("Failed to read secret name");
        let secret = secret.trim();

        println!("Enter the secret: ");
        io::stdin().read_line(&mut name).expect("Failed to read secret");
        let name = name.trim();

        let result = store(secret.to_string(), Some("secret".to_string()), Some(name.to_string()), Some("/personal".to_string()), None).await;

        println!("{:?}", result);
    } else if service == "update" {
        let mut secret_id = String::new();

        println!("Enter the secret id: ");
        io::stdin().read_line(&mut secret_id).expect("Failed to read secret id");
        let secret_id = secret_id.trim();

        let result = update(secret_id.to_string(), Some("secret-name".to_string()), Some("".to_string()), Some("".to_string()), Some("".to_string())).await;

        if let Ok(json_result) = result{
            let result: Value = serde_json::from_str(&json_result).unwrap();        
            println!("Status: {:?}", result["status"].as_str().unwrap());
            println!("Summary: {:?}", result["summary"].as_str().unwrap());
        }
    } else {
        println!("Invalid service");
    }
 
    

}
