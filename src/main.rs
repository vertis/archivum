use duct::cmd;
use serde_json::Value;
use std::path::Path;

use clap::{arg, command, value_parser};
use std::path::PathBuf;

fn main() {
    let matches = command!("live-mirror")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Mirrors GitHub repositories for a specified user or organization")
        .subcommand(
            command!("download")
                .about("Downloads repositories for the specified user or organization")
                .arg(
                    arg!(
                        -u --"user-org" <USER_OR_ORG> "Specifies the GitHub user or organization"
                    )
                    .required(true)
                    .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(
                        -b --basedir <BASE_OUTPUT_DIR> "Specifies the base output directory where repositories will be mirrored"
                    )
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            command!("upload")
                .about("Uploads mirrored repositories to a specified destination")
                .arg(
                    arg!(
                        -d --destination <DESTINATION> "Specifies the destination for uploading repositories"
                    )
                    .required(true)
                    .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(
                        -p --path <PATH> "Specifies the path of the mirrored repositories to upload"
                    )
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("download", sub_matches)) => {
            let user_or_org = sub_matches.get_one::<String>("user-org").expect("required");
            let base_output_dir = sub_matches.get_one::<PathBuf>("basedir").expect("required");
            let output_dir = format!("{}/{}", base_output_dir.display(), user_or_org);

            let repos = get_repositories(user_or_org).unwrap_or_else(|e| {
                eprintln!("Failed to list repositories: {}", e);
                std::process::exit(1);
            });

            process_repositories(&repos, &output_dir, user_or_org);
        }
        Some(("upload", sub_matches)) => {
            let destination = sub_matches
                .get_one::<String>("destination")
                .expect("required");
            let path = sub_matches.get_one::<PathBuf>("path").expect("required");
            println!(
                "Uploading repositories from {} to {}",
                path.display(),
                destination
            );
            // Placeholder for upload logic
        }
        _ => eprintln!("No valid subcommand was used."),
    }
}

fn process_repositories(repos: &[String], output_dir: &str, user_or_org: &str) {
    for repo in repos {
        println!("Processing repository: {}/{}", user_or_org, repo);
        process_repository(repo, output_dir, user_or_org);
    }
}

fn get_repositories(user_or_org: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = cmd!("gh", "repo", "list", user_or_org, "--json", "name").read()?;
    let repos = serde_json::from_str::<Vec<Value>>(&output)?
        .into_iter()
        .map(|repo| {
            repo["name"]
                .as_str()
                .expect("Expected a string")
                .to_string()
        })
        .collect::<Vec<String>>();
    Ok(repos)
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
