use reqwest::blocking::Client;

pub fn create_org(url: &str, token: &str, org_name: &str) -> bool {
    let client = Client::new();
    let new_org = serde_json::json!({
        "username": org_name,
        "full_name": format!("{} Full Name", org_name),
        "description": format!("{} is a great organization.", org_name),
        "website": "",
        "location": "World",
        "visibility": "private",
    });

    let res = client
        .post(format!("{}/api/v1/orgs", url))
        .bearer_auth(token)
        .json(&new_org)
        .send();

    matches!(res, Ok(response) if response.status().is_success())
}

pub fn check_repo_exists(url: &str, token: &str, org_name: &str, repo_name: &str) -> bool {
    let client = Client::new();
    let res = client
        .get(format!("{}/api/v1/repos/{}/{}", url, org_name, repo_name))
        .bearer_auth(token)
        .send();

    matches!(res, Ok(response) if response.status().is_success())
}

pub fn create_repo(url: &str, token: &str, org_name: &str, repo_name: &str) -> bool {
    let client = Client::new();
    let new_repo = serde_json::json!({
        "name": repo_name,
        "description": format!("{} is a great repository.", repo_name),
        "private": true,
    });

    let res = client
        .post(format!("{}/api/v1/orgs/{}/repos", url, org_name))
        .bearer_auth(token)
        .json(&new_repo)
        .send();

    matches!(res, Ok(response) if response.status().is_success())
}

pub fn check_user_or_org_exists(url: &str, token: &str, name: &str) -> bool {
    let client = Client::new();
    let user_res = client
        .get(format!("{}/api/v1/users/{}", url, name))
        .bearer_auth(token)
        .send();

    let org_res = client
        .get(format!("{}/api/v1/orgs/{}", url, name))
        .bearer_auth(token)
        .send();

    user_res.is_ok() && user_res.unwrap().status().is_success() ||
    org_res.is_ok() && org_res.unwrap().status().is_success()
}

pub fn create_org_if_no_conflict(url: &str, token: &str, org_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if check_user_or_org_exists(url, token, org_name) {
        Ok(false) // Organization already exists
    } else {
        Ok(create_org(url, token, org_name))
    }
}

