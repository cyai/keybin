use reqwest;
use serde_json::{json, Value};
// use dotenv::dotenv;

pub async fn update(
    _id: String,
    _name: Option<String>,
    _folder: Option<String>,
    _metadata: Option<String>,
    _tags: Option<String>,
    _access_token: &str
) -> Result<String, reqwest::Error> {
    // dotenv().ok();

    let url = "https://vault.aws.us.pangea.cloud/v1/update";

    // let access_token = std::env::var("VAULT_ACCESS_TOKEN").expect("VAULT_ACCESS_TOKEN must be set");

    let client = reqwest::Client::new();

    let mut payload = json!({"id": _id,});

    if let Some(name) = _name{
        payload["name"] = json!(name);
    }
     
    if let Some(folder) = _folder{
        payload["folder"] = json!(folder);
    }
    
    // if let Some(metadata) = _metadata{
    //     payload["metadata"] = json!(metadata);
    // }

    // if let Some(tags) = _tags{
    //     payload["tags"] = json!(tags);
    // } 

    // println!("Payload: {:?}", payload);

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", _access_token))
        .header("Content-Type",  "application/json")
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success(){
        let text = response.text().await?;
        let text: Value = serde_json::from_str(&text).unwrap();

        return Ok(text.to_string());
    } else {
        return Ok(format!("Response Failed: {:?}", response.status()));
    }

}