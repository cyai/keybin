use reqwest;
use serde_json::{json, Value};  
use dotenv::dotenv;



pub async fn get(_id: &str) -> Result<String, reqwest::Error> {
    dotenv().ok();
    let url = "https://vault.aws.us.pangea.cloud/v1/get";

    let client = reqwest::Client::new();

    let access_token = std::env::var("VAULT_ACCESS_TOKEN").expect("VAULT_ACCESS_TOKEN must be set");

    // println!("ID = {}", _id );
    let payload = json!({
        "id": _id
    });

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;


    if response.status().is_success(){
        let text = response.text().await?;
        let text: Value = serde_json::from_str(&text).unwrap();
        return Ok(text.to_string());
    } else {    
        println!("Response failed: {:?}", response.status());
        return Ok(format!("Response failed: {:?}", response.status()));
    }

}
