use duct::cmd;
use serde_json::Value;
use std::path::Path;

fn main() {
    let user_or_org = "vertis"; // Replace with the actual user or org name
    let base_output_dir = "mirror"; // Replace with the actual base output directory path
    let output_dir = format!("{}/{}", base_output_dir, user_or_org); // Construct the full output directory path

    let repos = get_repositories(user_or_org).unwrap_or_else(|e| {
        eprintln!("Failed to list repositories: {}", e);
        std::process::exit(1);
    });

    for repo in repos {
        println!("Processing repository: {}/{}", user_or_org, repo);
        process_repository(&repo, &output_dir, user_or_org);
    }
}

fn get_repositories(user_or_org: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

fn process_repository(repo: &str, output_dir: &str, user_or_org: &str) {
    let repo_path = format!("{}/{}.git", output_dir, repo);
    if Path::new(&repo_path).exists() {
        update_repository(&repo_path, repo);
    } else {
        clone_repository(user_or_org, repo, &repo_path);
    }
}

fn update_repository(repo_path: &str, repo: &str) {
    if let Err(e) = cmd!("git", "--git-dir", repo_path, "fetch", "--all").run() {
        eprintln!("Failed to fetch changes for repository {}: {}", repo, e);
    }
}

fn clone_repository(user_or_org: &str, repo: &str, repo_path: &str) {
    if let Err(e) = cmd!(
        "git",
        "clone",
        "--mirror",
        &format!("https://github.com/{}/{}.git", user_or_org, repo),
        repo_path
    )
    .run()
    {
        eprintln!("Failed to clone repository {}: {}", repo, e);
    }
}
