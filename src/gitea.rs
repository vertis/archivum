use crate::config::GiteaConfig;
use duct::cmd;
use reqwest::blocking::Client;
use url::Url;

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

    user_res.is_ok() && user_res.unwrap().status().is_success()
        || org_res.is_ok() && org_res.unwrap().status().is_success()
}

pub fn create_org_if_no_conflict(
    url: &str,
    token: &str,
    org_name: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    if check_user_or_org_exists(url, token, org_name) {
        Ok(false) // Organization already exists
    } else {
        Ok(create_org(url, token, org_name))
    }
}

/// Ensure that a repository exists in Gitea by creating the organization and repository if they don't exist.
pub fn ensure_repo_exists(
    config: &GiteaConfig,
    user_or_org: &str,
    repo: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // First, ensure the organization exists
    match create_org_if_no_conflict(&config.url, &config.token, user_or_org) {
        Ok(created) => {
            if created {
                println!("Created new organization in Gitea: {}", user_or_org);
            }
        }
        Err(e) => {
            return Err(format!(
                "Failed to create organization in Gitea: {}: {}",
                user_or_org, e
            )
            .into());
        }
    }

    // Then, check if the repository exists and create it if it doesn't
    if !check_repo_exists(&config.url, &config.token, user_or_org, repo) {
        if create_repo(&config.url, &config.token, user_or_org, repo) {
            println!("Created new repository in Gitea: {}/{}", user_or_org, repo);
        } else {
            return Err(format!(
                "Failed to create repository in Gitea: {}/{}",
                user_or_org, repo
            )
            .into());
        }
    }
    Ok(())
}

/// Push a repository to Gitea using the provided configuration.
pub fn push(
    config: &GiteaConfig,
    repo_path: &str,
    org_name: &str,
    repo_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut push_url = Url::parse(&config.url)?;
    {
        let mut segments = push_url
            .path_segments_mut()
            .map_err(|_| "Gitea URL must be absolute")?;
        segments.pop_if_empty();
        let repo_segment = format!("{}.git", repo_name);
        segments.extend([org_name, repo_segment.as_str()]);
    }

    push_url
        .set_username(&config.username)
        .map_err(|_| "invalid username for Gitea URL")?;
    push_url
        .set_password(Some(&config.password))
        .map_err(|_| "invalid password for Gitea URL")?;

    let authenticated_url: String = push_url.into();
    cmd!(
        "git",
        "--git-dir",
        repo_path,
        "push",
        "--mirror",
        authenticated_url
    )
    .run()?;
    Ok(())
}
