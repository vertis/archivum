use crate::actions;
use crate::config::Config;
use crate::github::{get_repositories, get_starred_repositories};
use std::path::Path;

fn process_user_or_org(
    user_or_org: &str,
    output_dir: &Path,
    new_only: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let repos = get_repositories(user_or_org)?;
    actions::process_repositories(
        &repos,
        &output_dir.join(user_or_org).to_string_lossy(),
        user_or_org,
        None,
        new_only,
    )?;
    Ok(())
}

fn process_individual_repo(
    full_repo_name: &str,
    output_dir: &Path,
    new_only: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let split: Vec<&str> = full_repo_name.split('/').collect();
    if split.len() == 2 {
        let user_or_org = split[0];
        let repo = split[1];
        actions::process_repositories(
            &[repo.to_string()],
            &output_dir.join(user_or_org).to_string_lossy(),
            user_or_org,
            None,
            new_only,
        )?;
    } else {
        eprintln!("Invalid repository name format: {}", full_repo_name);
    }
    Ok(())
}

pub fn execute(
    matches: &clap::ArgMatches,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new(&config.output_dir);
    let new_only = matches.get_flag("new-only");

    // Process users and organizations
    for user_or_org in config.users.iter().chain(config.organizations.iter()) {
        process_user_or_org(user_or_org, output_dir, new_only)?;
    }

    // Process individual repositories
    for full_repo_name in &config.repositories {
        process_individual_repo(full_repo_name, output_dir, new_only)?;
    }

    // Process starred repositories if enabled in config
    if config.include_starred {
        println!("Processing starred repositories (enabled in config):");
        let starred_repos = get_starred_repositories()?;
        for full_repo_name in &starred_repos {
            println!("{}", full_repo_name);
            process_individual_repo(full_repo_name, output_dir, new_only)?;
        }
    }

    Ok(())
}
