use reqwest;
use serde_json::{json, Value};
// use dotenv::dotenv;

pub async fn list(
    _folder: Option<String>,
    _tags: Option<String>,
    _name_contains: Option<String>,
    _created_at: Option<String>,
    _size: Option<i32>,
    _order: Option<String>,
    _order_by: Option<String>,
    _last: Option<String>,
    _include_secrets: Option<bool>,
    _access_token: &str,
) -> Result<String, reqwest::Error> {
    // dotenv().ok();

    let url = "https://vault.aws.us.pangea.cloud/v1/list";

    let client = reqwest::Client::new();

    // let access_token = std::env::var("VAULT_ACCESS_TOKEN").expect("VAULT_ACCESS_TOKEN must be set");

    let mut payload = json!({});

    if let Some(folder) = _folder {
        payload["filter"]["folder"] = json!(folder);
    }

    if let Some(tags) = _tags {
        payload["filter"]["tags"] = json!(tags);
    }

    if let Some(name_contains) = _name_contains {
        payload["filter"]["name__contains"] = json!(name_contains);
    }

    if let Some(created_at) = _created_at {
        payload["filter"]["created_at__gt"] = json!(created_at);
    }

    if let Some(size) = _size {
        payload["size"] = json!(size);
    }

    if let Some(order) = _order {
        payload["order"] = json!(order);
    }

    if let Some(order_by) = _order_by {
        payload["order_by"] = json!(order_by);
    }

    if let Some(last) = _last {
        payload["last"] = json!(last);
    }

    if let Some(include_secrets) = _include_secrets {
        payload["include_secrets"] = json!(include_secrets);
    }

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", _access_token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        let text = response.text().await?;
        let text: Value = serde_json::from_str(&text).unwrap();
        return Ok(text.to_string());
    } else {
        return Ok(response.status().to_string());
    }

}
