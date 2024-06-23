use crate::gitea::{
    check_repo_exists, create_org_if_no_conflict, create_repo, mirror_push_repo,
};
use clap::ArgMatches;
use glob::glob;
use std::path::PathBuf;

pub async fn execute(sub_matches: &ArgMatches) {
    let destination = sub_matches.get_one::<String>("destination").expect("required");
    let path = sub_matches.get_one::<PathBuf>("path").expect("required");
    let token = "bf88e1f7759c52fdd08d3fc8e4105f1a0a689987"; // This should be securely retrieved or passed as an argument

    // Glob pattern to match all git repositories in the specified path
    let pattern = format!("{}/**/*.git", path.display());
    let repos = glob(&pattern).expect("Failed to read glob pattern");

    for entry in repos {
        match entry {
            Ok(repo_path) => {
                let mut repo_name = repo_path.file_name().unwrap().to_str().unwrap();
                let org_name = repo_path.parent().unwrap().file_name().unwrap().to_str().unwrap();

                // Strip the .git from the end of repo_name if it exists
                if repo_name.ends_with(".git") {
                    repo_name = &repo_name[..repo_name.len() - 4];
                }

                println!("Processing repository: {}", repo_name);

                // Check if the organization exists, create if not
                if let Err(e) = create_org_if_no_conflict(destination, token, org_name).await {
                    eprintln!("Failed to create organization {}: {}", org_name, e);
                    continue;
                }

                // Check if the repository exists within the organization, create if not
                if !check_repo_exists(destination, token, org_name, repo_name).await {
                    if create_repo(destination, token, org_name, repo_name).await {
                        println!("Repository {}/{} created successfully.", org_name, repo_name);
                    } else {
                        eprintln!("Failed to create repository {}/{}.", org_name, repo_name);
                        continue;
                    }
                }

                // Mirror push the repository
                if let Err(e) = mirror_push_repo(&repo_path, destination, org_name, repo_name).await {
                    eprintln!("Failed to mirror push repository {}/{}: {}", org_name, repo_name, e);
                } else {
                    println!("Successfully mirrored repository {}/{} to destination.", org_name, repo_name);
                }
            },
            Err(e) => eprintln!("Error processing entry: {}", e),
        }
    }
}
