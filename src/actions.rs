use crate::config::GiteaConfig;
use crate::git;
use crate::gitea;
use crate::github;
use std::path::Path;

/// Process a list of repositories by cloning or updating them from GitHub and pushing to Gitea.
pub fn process_repositories(
    repos: &[String],
    output_dir: &str,
    user_or_org: &str,
    gitea_config: Option<&GiteaConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    for repo in repos {
        println!("Processing repository: {}/{}", user_or_org, repo);
        process_repository(repo, output_dir, user_or_org, gitea_config)?;
    }
    Ok(())
}

/// Process a single repository by cloning or updating it from GitHub and pushing to Gitea.
fn process_repository(
    repo: &str,
    output_dir: &str,
    user_or_org: &str,
    gitea_config: Option<&GiteaConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = format!("{}/{}.git", output_dir, repo);
    let repo_dir = Path::new(&repo_path);

    if repo_dir.exists() {
        git::update_repo_with_lfs(&repo_path, repo)?;
    } else {
        github::clone_with_mirror(user_or_org, repo, &repo_path)?;
    }

    if let Some(config) = gitea_config {
        gitea::ensure_repo_exists(config, user_or_org, repo)?;
        gitea::push(config, &repo_path, user_or_org, repo)?;
    }

    Ok(())
}
