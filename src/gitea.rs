use reqwest::Client;

pub async fn create_org(api_url: &str, token: &str, org_name: &str) -> bool {
    let client = Client::new();
    let new_org = serde_json::json!({
        "username": org_name,
        "full_name": format!("{} Full Name", org_name),
        "description": format!("{} is a great organization.", org_name),
        "website": "",
        "location": "World",
        "visibility": "private",
    });

    let body = serde_json::to_string(&new_org).unwrap();

    let res = client
        .post(format!("{}/api/v1/orgs", api_url))
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;

    match res {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}
