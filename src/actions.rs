use duct::cmd;
use std::path::Path;

pub fn process_repositories(repos: &[String], output_dir: &str, user_or_org: &str) {
    for repo in repos {
        println!("Processing repository: {}/{}", user_or_org, repo);
        process_repository(repo, output_dir, user_or_org);
    }
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
    // Handle LFS objects after fetching changes
    if let Err(e) = cmd!("git", "lfs", "fetch", "--all", repo_path).run() {
        eprintln!("Failed to fetch LFS objects for repository {}: {}", repo, e);
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
    // Initialize and fetch LFS objects after cloning
    if let Err(e) = cmd!("git", "lfs", "install").run() {
        eprintln!(
            "Failed to initialize Git LFS for repository {}: {}",
            repo, e
        );
    }
    if let Err(e) = cmd!("git", "lfs", "fetch", "--all", repo_path).run() {
        eprintln!("Failed to fetch LFS objects for repository {}: {}", repo, e);
    }
}
