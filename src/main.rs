mod core {
    pub mod commands {
        pub mod get;
        pub mod list;
        pub mod store;
        pub mod update;
        pub mod delete;
    }
    pub mod db;
}

mod cli;

use crate::core::{commands::{get::get, list::list, store::store, update::update, delete::delete}, db};
use serde_json::Value;
use cli::{Keybinargs, SubCommand};
use clap::Parser;
use terminal_clipboard::set_string;

#[tokio::main]
async fn main(){
    let _ = db::create_db().await;



    let args = Keybinargs::parse();

    match args.subcmd {

    SubCommand::Get(_get)=>{

        let result = db::get_secret_id(&_get.name).await;

        if let Ok(id) = result {
            let _id = id.clone();
            println!("Secret ID: {:?}", &_id.unwrap().to_string());
        
        
            let result = get(&id.unwrap().to_string()).await;
            
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
                    let secret = result["result"]["current_version"]["secret"].as_str().unwrap_or("-");

                    set_string(secret).unwrap();
                    println!("Secret copied to clipboard");
                    
                    assert_eq!(secret, terminal_clipboard::get_string().unwrap());

                } 
            }
        } else {
            println!("{:?}", result.unwrap_err());
            println!("Error getting secret ID from the local data (Try Again by running store command!)");
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
                        let id = item["id"].as_str().unwrap_or("-");
                        let key_type = item["type"].as_str().unwrap_or("-");
                        let created = item["created_at"].as_str().unwrap_or("-");
                        let state = item["current_version"]["state"].as_str().unwrap_or("-");
        
                        println!("Name: {} \nSecret ID: {} \nType: {} \nCreated: {} \nState: {}\n", name, id, key_type, created, state);
                        println!("-------------------------------------------------");
                    }
                } else {
                    println!("No items found");
                }
            }
        }
    },

    SubCommand::Store(_store) => {
        
        let secret = _store.value;
        let key_type = _store.key_type;
        let name = _store.name;
        let description = _store.description;
        let tags = _store.tags;

        let _ = db::create_db().await;
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

                // update the db
                let secrets = list(None, None, None, None, None, None, None, None, None).await;

             // store the name and id in the db
                let result: Value = serde_json::from_str(&secrets.unwrap()).unwrap();

                if let Some(items) = result["result"]["items"].as_array(){
                    for item in items{
                        let name = item["name"].as_str().unwrap_or("-");
                        let id = item["id"].as_str().unwrap_or("-");
                        let _ = db::insert_secret(name, id).await;

                        // if let Ok(_) = secret_result {
                        //  println!("Secret inserted into db");
                        // } else {
                        //     println!("Error: {:?}", secret_result.unwrap_err());
                        // }
                    }
                }
            }
        }        
    },

    SubCommand::Update(_update) => {
        let id = _update.secret_id;
        let name = _update.name;
        let folder = _update.folder;
        let metadata = _update.metadata;
        let tags = _update.tags;
        let _name = name.clone();
        let _id = id.clone();

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
    },

    SubCommand::GetId(_get_id) => {
        let result = db::get_secret_id(&_get_id.name).await;

        if let Ok(id) = result {
            println!("Secret ID: {:?}", id);
        } else {
            println!("Error: {:?}", result.unwrap_err());
        }
    },
 } 

}
