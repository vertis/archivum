use crate::actions;
use crate::config::Config;
use crate::github::get_repositories;
use std::path::Path;

fn process_user_or_org(
    user_or_org: &str,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let repos = get_repositories(user_or_org)?;
    actions::process_repositories(
        &repos,
        &output_dir.join(user_or_org).to_string_lossy(),
        user_or_org,
        None,
    )?;
    Ok(())
}

fn process_individual_repo(
    full_repo_name: &str,
    output_dir: &Path,
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
        )?;
    } else {
        eprintln!("Invalid repository name format: {}", full_repo_name);
    }
    Ok(())
}

pub fn execute(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new(&config.output_dir);

    // Process users and organizations
    for user_or_org in config.users.iter().chain(config.organizations.iter()) {
        process_user_or_org(user_or_org, output_dir)?;
    }

    // Process individual repositories
    for full_repo_name in &config.repositories {
        process_individual_repo(full_repo_name, output_dir)?;
    }

    Ok(())
}
