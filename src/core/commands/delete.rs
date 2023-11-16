use reqwest;
use serde_json::{json, Value};
// use dotenv::dotenv;

pub async fn delete(_id: String, _access_token: &str) -> Result<String, reqwest::Error>{
    // dotenv().ok();

    let url = "https://vault.aws.us.pangea.cloud/v1/delete";

    let client = reqwest::Client::new();

    // let access_token = std::env::var("VAULT_ACCESS_TOKEN").expect("VAULT_ACCESS_TOKEN must be set");

    let payload = json!({"id": _id});

    let response = client
        .post(url)
        .header("Authrization", format!("Bearer {}", _access_token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    // println!("Response: {:?}", response);
    if response.status().is_success(){
        let text = response.text().await?;
        let text: Value = serde_json::from_str(&text).unwrap();

        return Ok(text.to_string());
    } else {
        return Ok(format!("Response Failed: {:?}", response.status()));
    }
}