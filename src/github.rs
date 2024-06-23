use duct::cmd;
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
    let output = cmd!("gh", "repo", "list", user_or_org, "--json", "name").read()?;
    let repos = serde_json::from_str::<Vec<Value>>(&output)?
        .into_iter()
        .map(|repo| {
            repo["name"]
                .as_str()
                .expect("Expected a string")
                .to_string()
        })
        .collect::<Vec<String>>();
    Ok(repos)
}
