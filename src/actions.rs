use duct::cmd;
use std::path::Path;
use crate::config::GiteaConfig;
use crate::gitea;

pub fn process_repositories(repos: &[String], output_dir: &str, user_or_org: &str, gitea_config: Option<&GiteaConfig>) -> Result<(), Box<dyn std::error::Error>> {
    for repo in repos {
        println!("Processing repository: {}/{}", user_or_org, repo);
        process_repository(repo, output_dir, user_or_org, gitea_config)?;
    }
    Ok(())
}

fn process_repository(repo: &str, output_dir: &str, user_or_org: &str, gitea_config: Option<&GiteaConfig>) -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = format!("{}/{}.git", output_dir, repo);
    let repo_dir = Path::new(&repo_path);

    if repo_dir.exists() {
        update_repository(&repo_path, repo)?;
    } else {
        clone_from_github(user_or_org, repo, &repo_path)?;
    }

    if let Some(config) = gitea_config {
        ensure_gitea_repo_exists(config, user_or_org, repo)?;
        push_to_gitea(config, &repo_path, user_or_org, repo)?;
    }

    Ok(())
}

fn ensure_gitea_repo_exists(config: &GiteaConfig, user_or_org: &str, repo: &str) -> Result<(), Box<dyn std::error::Error>> {
    // First, ensure the organization exists
    match gitea::create_org_if_no_conflict(&config.url, &config.token, user_or_org) {
        Ok(created) => {
            if created {
                println!("Created new organization in Gitea: {}", user_or_org);
            }
        },
        Err(e) => {
            return Err(format!("Failed to create organization in Gitea: {}: {}", user_or_org, e).into());
        }
    }

    // Then, check if the repository exists and create it if it doesn't
    if !gitea::check_repo_exists(&config.url, &config.token, user_or_org, repo) {
        if gitea::create_repo(&config.url, &config.token, user_or_org, repo) {
            println!("Created new repository in Gitea: {}/{}", user_or_org, repo);
        } else {
            return Err(format!("Failed to create repository in Gitea: {}/{}", user_or_org, repo).into());
        }
    }
    Ok(())
}

fn clone_from_github(user_or_org: &str, repo: &str, repo_path: &str) -> Result<(), Box<dyn std::error::Error>> {
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

fn update_repository(repo_path: &str, _repo: &str) -> Result<(), Box<dyn std::error::Error>> {
    cmd!("git", "--git-dir", repo_path, "fetch", "--all").run()?;
    
    // Handle LFS objects after fetching changes
    cmd!("git", "lfs", "fetch", "--all", repo_path).run()?;

    Ok(())
}

pub fn push_to_gitea(config: &GiteaConfig, repo_path: &str, org_name: &str, repo_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let authenticated_url = format!("http://{}:{}@{}/{}/{}.git", 
        config.username, 
        config.password, 
        config.url.trim_start_matches("http://"),
        org_name, 
        repo_name
    );
    cmd!("git", "--git-dir", repo_path, "push", "--mirror", authenticated_url).run()?;
    Ok(())
}
