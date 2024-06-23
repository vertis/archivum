use duct::cmd;
use std::process::Command;
use serde_json::Value;

pub fn get_starred_repositories() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = cmd!(
        "gh",
        "api",
        "/user/starred?per_page=100",
        "-q",
        ".[].full_name"
    )
    .read()?;
    let repos = output
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    Ok(repos)
}

pub fn get_repositories(user_or_org: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = Command::new("gh")
        .args(&["api", &format!("users/{}/repos", user_or_org), "--paginate"])
        .output()?;

    if !output.status.success() {
        return Err(format!("GitHub CLI command failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let repos: Value = serde_json::from_str(&stdout)?;

    if let Value::Array(repos) = repos {
        let repos = repos
            .into_iter()
            .filter_map(|repo| repo["name"].as_str().map(|s| s.to_string()))
            .collect();
        Ok(repos)
    } else {
        Err("Unexpected response format from GitHub API".into())
    }
}
