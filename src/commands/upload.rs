use crate::config::{Config, GiteaConfig};
use crate::gitea::{check_repo_exists, create_org_if_no_conflict, create_repo};
use glob::glob;
use std::path::Path;

pub fn execute(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new(&config.output_dir);

    if let Some(gitea_config) = &config.gitea {
        process_gitea_tasks(config, output_dir, gitea_config)?;
    } else {
        return Err("Gitea configuration is missing".into());
    }

    Ok(())
}

fn process_gitea_tasks(
    config: &Config,
    output_dir: &Path,
    gitea_config: &GiteaConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    // Glob pattern to match all git repositories in the specified path
    let pattern = format!("{}/**/*.git", output_dir.display());
    let repos = glob(&pattern)?;

    for entry in repos {
        let repo_path = entry?;
        let repo_name = repo_path.file_stem().unwrap().to_str().unwrap();
        let org_name = repo_path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        println!("Processing repository: {}/{}", org_name, repo_name);

        // Check if the organization exists, create if not
        match create_org_if_no_conflict(&gitea_config.url, &gitea_config.token, org_name) {
            Ok(created) => {
                if created {
                    println!("Created organization {} in Gitea.", org_name);
                } else {
                    println!("Organization {} already exists in Gitea.", org_name);
                }
            }
            Err(e) => {
                eprintln!(
                    "Error while checking/creating organization {}: {}",
                    org_name, e
                );
                // Continue processing even if there's an error with organization creation
            }
        }

        // Ensure the repository exists within the organization, create if not
        if !check_repo_exists(&gitea_config.url, &gitea_config.token, org_name, repo_name) {
            if create_repo(&gitea_config.url, &gitea_config.token, org_name, repo_name) {
                println!(
                    "Repository {}/{} created successfully in Gitea.",
                    org_name, repo_name
                );
            } else {
                eprintln!(
                    "Failed to create repository {}/{} in Gitea.",
                    org_name, repo_name
                );
                continue;
            }
        }

        // Push the repository to Gitea
        match crate::actions::push_to_gitea(
            gitea_config,
            &repo_path.to_string_lossy(),
            org_name,
            repo_name,
        ) {
            Ok(_) => println!(
                "Successfully pushed repository {}/{} to Gitea.",
                org_name, repo_name
            ),
            Err(e) => eprintln!(
                "Failed to push repository {}/{} to Gitea: {}",
                org_name, repo_name, e
            ),
        }
    }

    Ok(())
}
