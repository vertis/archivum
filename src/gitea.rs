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

pub async fn check_org_exists(api_url: &str, token: &str, org_name: &str) -> bool {
    let client = Client::new();
    let res = client
        .get(format!("{}/api/v1/orgs/{}", api_url, org_name))
        .bearer_auth(token)
        .send()
        .await;

    matches!(res, Ok(response) if response.status().is_success())
}

pub async fn check_repo_exists(api_url: &str, token: &str, org_name: &str, repo_name: &str) -> bool {
    let client = Client::new();
    let res = client
        .get(format!(
            "{}/api/v1/repos/{}/{}",
            api_url, org_name, repo_name
        ))
        .bearer_auth(token)
        .send()
        .await;

    matches!(res, Ok(response) if response.status().is_success())
}

pub async fn create_repo(api_url: &str, token: &str, org_name: &str, repo_name: &str) -> bool {
    let client = Client::new();
    let new_repo = serde_json::json!({
        "name": repo_name,
        "description": format!("{} is a great repository.", repo_name),
        "private": true,
    });

    let body = serde_json::to_string(&new_repo).unwrap();

    let res = client
        .post(format!("{}/api/v1/orgs/{}/repos", api_url, org_name))
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;

    match res {
        Ok(response) => {
            let status_success = response.status().is_success();
            if let Ok(text) = response.text().await {
                eprintln!(
                    "Trying to create repository {}/{}. Response body: {}",
                    org_name, repo_name, text
                );
            }
            status_success
        }
        Err(e) => {
            eprintln!(
                "Failed to create repository {}/{}. Response body: {}",
                org_name,
                repo_name,
                e.to_string()
            );
            false
        }
    }
}

pub async fn check_user_or_org_exists(
    api_url: &str,
    token: &str,
    name: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = Client::new();
    let user_res = client
        .get(format!("{}/api/v1/users/{}", api_url, name))
        .bearer_auth(token)
        .send()
        .await;

    let org_res = client
        .get(format!("{}/api/v1/orgs/{}", api_url, name))
        .bearer_auth(token)
        .send()
        .await;

    if user_res.is_ok() && user_res.unwrap().status().is_success() {
        return Ok(true); // User exists
    }

    if org_res.is_ok() && org_res.unwrap().status().is_success() {
        return Ok(true); // Org exists
    }

    Ok(false) // Neither user nor org exists
}

pub async fn create_org_if_no_conflict(
    api_url: &str,
    token: &str,
    org_name: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    if check_user_or_org_exists(api_url, token, org_name).await? {
        Err("A user or organization with the same name already exists.".into())
    } else {
        Ok(create_org(api_url, token, org_name).await)
    }
}

pub async fn mirror_push_repo(
    repo_path: &PathBuf,
    api_url: &str,
    org_name: &str,
    repo_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo_url = format!("{}/{}/{}", api_url, org_name, repo_name);
    let authenticated_repo_url = format!("http://vertis:4rch1v1st@{}", &repo_url[7..]);
    let _output = cmd!("git", "push", "--mirror", &authenticated_repo_url)
        .dir(repo_path)
        .run()?;
    Ok(())
}
