use duct::cmd;
use serde_json::Value;
use std::process::Command;

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
        return Err(format!(
            "GitHub CLI command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
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

// clone with --mirror and lfs, we should probably make this more generic at some point
pub fn clone_with_mirror(
    user_or_org: &str,
    repo: &str,
    repo_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    cmd!(
        "git",
        "clone",
        "--mirror",
        &format!("https://github.com/{}/{}.git", user_or_org, repo),
        repo_path
    )
    .run()?;

    // Initialize and fetch LFS objects after cloning
    cmd!("git", "lfs", "install").run()?;
    cmd!("git", "lfs", "fetch", "--all", repo_path).run()?;

    Ok(())
}
