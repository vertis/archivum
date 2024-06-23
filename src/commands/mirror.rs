use crate::config::{Config, GiteaConfig};
use crate::actions;
use crate::github::get_repositories;
use std::path::Path;
use crate::gitea::{create_org_if_no_conflict, check_repo_exists, create_repo};

pub fn execute(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new(&config.output_dir);

    // Process users and organizations
    for user_or_org in config.users.iter().chain(config.organizations.iter()) {
        process_user_or_org(user_or_org, output_dir, config.gitea.as_ref())?;
    }

    // Process individual repositories
    for full_repo_name in &config.repositories {
        process_individual_repo(full_repo_name, output_dir, config.gitea.as_ref())?;
    }

    // Create Gitea organizations and repositories if configuration is provided
    if let Some(gitea_config) = &config.gitea {
        process_gitea_tasks(config, output_dir, gitea_config)?;
    }

    Ok(())
}

fn process_user_or_org(
    user_or_org: &str,
    output_dir: &Path,
    gitea_config: Option<&GiteaConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    let repos = get_repositories(user_or_org)?;
    actions::process_repositories(&repos, &output_dir.join(user_or_org).to_string_lossy(), user_or_org, gitea_config)?;
    Ok(())
}

fn process_individual_repo(
    full_repo_name: &str,
    output_dir: &Path,
    gitea_config: Option<&GiteaConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    let split: Vec<&str> = full_repo_name.split('/').collect();
    if split.len() == 2 {
        let user_or_org = split[0];
        let repo = split[1];
        actions::process_repositories(&[repo.to_string()], &output_dir.join(user_or_org).to_string_lossy(), user_or_org, gitea_config)?;
    } else {
        eprintln!("Invalid repository name format: {}", full_repo_name);
    }
    Ok(())
}

fn process_gitea_tasks(
    config: &Config,
    output_dir: &Path,
    gitea_config: &GiteaConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    for user_or_org in config.users.iter().chain(config.organizations.iter()) {
        // Check if the organization exists, create if not
        match create_org_if_no_conflict(&gitea_config.url, &gitea_config.token, user_or_org) {
            Ok(created) => {
                if created {
                    println!("Created organization {} in Gitea.", user_or_org);
                } else {
                    println!("Organization {} already exists in Gitea.", user_or_org);
                }
            },
            Err(e) => {
                eprintln!("Error while checking/creating organization {}: {}", user_or_org, e);
                // Continue processing even if there's an error with organization creation
            }
        }

        let repos = get_repositories(user_or_org)?;
        for repo in repos {
            process_gitea_repo(gitea_config, user_or_org, &repo, output_dir)?;
        }
    }

    // Process individual repositories for Gitea upload
    for full_repo_name in &config.repositories {
        let split: Vec<&str> = full_repo_name.split('/').collect();
        if split.len() == 2 {
            let user_or_org = split[0];
            let repo = split[1];
            process_gitea_repo(gitea_config, user_or_org, repo, output_dir)?;
        }
    }

    Ok(())
}

fn process_gitea_repo(
    gitea_config: &GiteaConfig,
    user_or_org: &str,
    repo: &str,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the repository exists within the organization, create if not
    if !check_repo_exists(&gitea_config.url, &gitea_config.token, user_or_org, repo) {
        if create_repo(&gitea_config.url, &gitea_config.token, user_or_org, repo) {
            println!("Repository {}/{} created successfully in Gitea.", user_or_org, repo);
        } else {
            return Err(format!("Failed to create repository {}/{} in Gitea.", user_or_org, repo).into());
        }
    }

    // Push the repository to Gitea
    let repo_path = output_dir.join(user_or_org).join(format!("{}.git", repo));
    actions::push_to_gitea(gitea_config, &repo_path.to_string_lossy(), user_or_org, repo)?;
    println!("Successfully pushed repository {}/{} to Gitea.", user_or_org, repo);

    Ok(())
}
