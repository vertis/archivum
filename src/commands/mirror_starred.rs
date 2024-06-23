use crate::actions;
use crate::github::get_starred_repositories;
use crate::config::Config;
use std::path::Path;

pub fn execute(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new(&config.output_dir);

    let starred_repos = get_starred_repositories()?;

    println!("Mirroring starred repositories:");
    let mut errors = Vec::new();

    for full_repo_name in &starred_repos {
        let split: Vec<&str> = full_repo_name.split('/').collect();
        if split.len() == 2 {
            let user_or_org = split[0];
            let repo = split[1];
            let repo_output_dir = output_dir.join(user_or_org);
            match actions::process_repositories(&[repo.to_string()], &repo_output_dir.to_string_lossy(), user_or_org, config.gitea.as_ref()) {
                Ok(_) => println!("Successfully mirrored {}/{}", user_or_org, repo),
                Err(e) => {
                    let error_msg = format!("Error processing repository {}/{}: {}", user_or_org, repo, e);
                    eprintln!("{}", error_msg);
                    errors.push(error_msg);
                }
            }
        } else {
            let error_msg = format!("Invalid repository name format: {}", full_repo_name);
            eprintln!("{}", error_msg);
            errors.push(error_msg);
        }
    }

    if !errors.is_empty() {
        eprintln!("\nEncountered {} error(s) while mirroring repositories:", errors.len());
        for (i, error) in errors.iter().enumerate() {
            eprintln!("{}. {}", i + 1, error);
        }
    }

    Ok(())
}
