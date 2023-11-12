
mod core {
    pub mod commands {
        pub mod get;
        pub mod list;
        pub mod store;
        pub mod update;
        pub mod delete;
    }
}

mod cli;

use core::commands::{get::get, list::list, store::store, update::update, delete::delete};
use serde_json::Value;
use cli::{Keybinargs, SubCommand};
use clap::Parser;


#[tokio::main]
async fn main(){
    let args = Keybinargs::parse();

    match args.subcmd {

    SubCommand::Get(_get)=>{
        
        let result = get(&_get.secret_id).await;
        
        if let Ok(json_result) = result {
            if json_result.contains("403") {
                println!("Deletion failed with a 403 error: Forbidden");
            } else if json_result.contains("400") {
                println!("Deletion failed with a 400 error: Bad Request");
            } else if json_result.contains("404") {
                println!("Deletion failed with a 404 error: Not Found");
            } else if json_result.contains("500") {
                println!("Deletion failed with a 500 error: Internal Server Error");
            } else {
                let result: Value = serde_json::from_str(&json_result).unwrap();
                let name = result["result"]["current_version"]["secret"].as_str().unwrap_or("-");
                println!("{:?}", name)
            } 
        }
    },

    SubCommand::List(_list) => {
        let folder = _list.folder;
        let tags = _list.tags;
        let name_contains = _list.name_contains;
        let created_at = _list.created_at;
        let size = _list.size;
        let order = _list.order;
        let order_by = _list.order_by;
        let last = _list.last;
        let include_secrets = _list.include_secrets;

        let result = list(
            folder.map(|s| s.to_string()),
            tags.map(|s| s.to_string()),
            name_contains.map(|s| s.to_string()),
            created_at.map(|s| s.to_string()),
            size.map(|s| s),
            order.map(|s| s.to_string()),
            order_by.map(|s| s.to_string()),
            last.map(|s| s.to_string()),
            include_secrets.map(|s| s),

        ).await;

        if let Ok(json_result) = result{
            if json_result.contains("403") {
                println!("Deletion failed with a 403 error: Forbidden");
            } else if json_result.contains("400") {
                println!("Deletion failed with a 400 error: Bad Request");
            } else if json_result.contains("404") {
                println!("Deletion failed with a 404 error: Not Found");
            } else if json_result.contains("500") {
                println!("Deletion failed with a 500 error: Internal Server Error");
            } else{
                let result: Value = serde_json::from_str(&json_result).unwrap();
                // let items = result["result"]["items"].as_array().ok_or("No items found");
                if let Some(items) = result["result"]["items"].as_array(){
                    for item in items{
                        let name = item["name"].as_str().unwrap_or("-");
                        let key_type = item["type"].as_str().unwrap_or("-");
                        let created = item["created_at"].as_str().unwrap_or("-");
                        let state = item["current_version"]["state"].as_str().unwrap_or("-");
        
                        println!("Name: {} \nType: {} \nCreated: {} \nState: {}\n", name, key_type, created, state);
                        println!("-------------------------------------------------");
                    }
                } else {
                    println!("No items found");
                }
            }
        }
    },

    SubCommand::Store(_store) => {
        let secret = _store.name;
        let key_type = _store.key_type;
        let name = _store.value;
        let description = _store.description;
        let tags = _store.tags;

        // println!("{:?}", Some(name.unwrap().to_string()));
        let result = store(
            secret.to_string(),
            key_type.map(|s| s.to_string()),
            name.map(|s| s.to_string()),
            description.map(|s| s.to_string()),
            tags.map(|s| s.to_string())
        ).await;

        // println!("{:?}", result);
        if let Ok(json_result) = result{
            if json_result.contains("403") {
                println!("Deletion failed with a 403 error: Forbidden");
            } else if json_result.contains("400") {
                println!("Deletion failed with a 400 error: Bad Request");
            } else if json_result.contains("404") {
                println!("Deletion failed with a 404 error: Not Found");
            } else if json_result.contains("500") {
                println!("Deletion failed with a 500 error: Internal Server Error");
            }
            else {
                let result: Value = serde_json::from_str(&json_result).unwrap();        
                println!("Status: {:?}", result["status"].as_str().unwrap_or("-"));
                println!("Summary: {:?}", result["summary"].as_str().unwrap_or("-"));
            }
        }

    },

    SubCommand::Update(_update) => {
        let id = _update.secret_id;
        let name = _update.name;
        let folder = _update.folder;
        let metadata = _update.metadata;
        let tags = _update.tags;

        let result = update(id, name.map(|s| s.to_string()),folder.map(|s| s.to_string()), metadata.map(|s| s.to_string()),tags.map(|s| s.to_string())).await;

        if let Ok(json_result) = result{
            if json_result.contains("403") {
                println!("Deletion failed with a 403 error: Forbidden");
            } else if json_result.contains("400") {
                println!("Deletion failed with a 400 error: Bad Request");
            } else if json_result.contains("404") {
                println!("Deletion failed with a 404 error: Not Found");
            } else if json_result.contains("500") {
                println!("Deletion failed with a 500 error: Internal Server Error");
            }
            else {
                let result: Value = serde_json::from_str(&json_result).unwrap();        
                println!("Status: {:?}", result["status"].as_str().unwrap_or("-"));
                println!("Summary: {:?}", result["summary"].as_str().unwrap_or("-"));
            }
        }

    },

    SubCommand::Delete(_delete) => {
        let result = delete(_delete.secret_id).await;
        

        if let Ok(json_result) = result {
                if json_result.contains("403") {
                    println!("Deletion failed with a 403 error: Forbidden");
                } else if json_result.contains("400") {
                    println!("Deletion failed with a 400 error: Bad Request");
                } else if json_result.contains("404") {
                    println!("Deletion failed with a 404 error: Not Found");
                } else if json_result.contains("500") {
                    println!("Deletion failed with a 500 error: Internal Server Error");
                }
                else {
                    let result: Value = serde_json::from_str(&json_result).unwrap();        
                    println!("Status: {:?}", result["status"].as_str().unwrap_or("-"));
                    println!("Summary: {:?}", result["summary"].as_str().unwrap_or("-"));
                }
            }
            
        
        // if let Ok(json_result) = result{
        //     let result: Value = serde_json::from_str(&json_result).unwrap();        
        //     println!("Status: {:?}", result["status"].as_str().unwrap_or("!! No status found !!"));
        //     println!("Summary: {:?}", result["summary"].as_str().unwrap_or("!! No summary found !!"));
        // }

    },
 } 

}
