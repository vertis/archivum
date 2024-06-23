use crate::actions;
use crate::github::get_starred_repositories;
use clap::ArgMatches;
use std::path::PathBuf;

pub fn execute(sub_matches: &ArgMatches) {
    let base_output_dir = sub_matches.get_one::<PathBuf>("basedir").expect("required");

    let starred_repos = get_starred_repositories().unwrap_or_else(|e| {
        eprintln!("Failed to list starred repositories: {}", e);
        std::process::exit(1);
    });

    println!("Starred repositories:");
    for full_repo_name in &starred_repos {
        println!("{}", full_repo_name);
        let split: Vec<&str> = full_repo_name.split('/').collect();
        if split.len() == 2 {
            let user_or_org = split[0];
            let repo = split[1];
            let output_dir = format!("{}/{}", base_output_dir.display(), user_or_org);
            actions::process_repositories(&[repo.to_string()], &output_dir, user_or_org);
        } else {
            eprintln!("Invalid repository name format: {}", full_repo_name);
        }
    }
}
