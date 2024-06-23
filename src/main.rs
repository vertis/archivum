mod commands;
mod gitea;
use clap::{arg, command, value_parser};
use std::path::PathBuf;
use crate::commands::{get_repositories, get_starred_repositories};
use crate::gitea::{create_org, create_org_if_no_conflict, check_org_exists, check_repo_exists, create_repo, mirror_push_repo};
use reqwest::Client;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
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
                command!("download-repo")
                    .about("Downloads a specific repository for the specified user or organization")
                    .arg(
                        arg!(
                            -u --"user-org" <USER_ORG> "Specifies the GitHub user or organization"
                        )
                        .required(true)
                        .value_parser(value_parser!(String)),
                    )
                    .arg(
                        arg!(
                            -r --repo <REPO_NAME> "Specifies the name of the repository to download"
                        )
                        .required(true)
                        .value_parser(value_parser!(String)),
                    )
                    .arg(
                        arg!(
                            -b --basedir <BASE_OUTPUT_DIR> "Specifies the base output directory where the repository will be mirrored"
                        )
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                    ),
            )
            .subcommand(
                command!("download-starred")
                    .about("Downloads starred repositories for the logged in user")
                    .arg(
                        arg!(
                            -b --basedir <BASE_OUTPUT_DIR> "Specifies the base output directory where starred repositories will be mirrored"
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

                commands::process_repositories(&repos, &output_dir, user_or_org);
            }
            Some(("download-repo", sub_matches)) => {
                let user_or_org = sub_matches.get_one::<String>("user-org").expect("required");
                let repo_name = sub_matches.get_one::<String>("repo").expect("required");
                let base_output_dir = sub_matches.get_one::<PathBuf>("basedir").expect("required");
                let output_dir = format!("{}/{}", base_output_dir.display(), user_or_org);

                println!("Processing single repository: {}/{}", user_or_org, repo_name);
                commands::process_repositories(&[repo_name.to_string()], &output_dir, user_or_org);
            },
            Some(("download-starred", sub_matches)) => {
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
                        commands::process_repositories(&[repo.to_string()], &output_dir, user_or_org);
                    } else {
                        eprintln!("Invalid repository name format: {}", full_repo_name);
                    }
                }
            },
            Some(("upload", sub_matches)) => {
                let destination = sub_matches.get_one::<String>("destination").expect("required");
                let path = sub_matches.get_one::<PathBuf>("path").expect("required");
                let token = "bf88e1f7759c52fdd08d3fc8e4105f1a0a689987"; // This should be securely retrieved or passed as an argument

                // Glob pattern to match all git repositories in the specified path
                let pattern = format!("{}/**/*.git", path.display());
                let repos = glob::glob(&pattern).expect("Failed to read glob pattern");

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
                            // if !check_org_exists(destination, token, org_name).await {
                            //     if create_org(destination, token, org_name).await {
                            //         println!("Organization {} created successfully.", org_name);
                            //     } else {
                            //         eprintln!("Failed to create organization {}.", org_name);
                            //         continue;
                            //     }
                            // }
                            create_org_if_no_conflict(destination, token, org_name).await;

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
                            if let Err(e) = gitea::mirror_push_repo(&repo_path, destination, org_name, repo_name).await {
                                eprintln!("Failed to mirror push repository {}/{}: {}", org_name, repo_name, e);
                            } else {
                                println!("Successfully mirrored repository {}/{} to destination.", org_name, repo_name);
                            }
                        },
                        Err(e) => eprintln!("Error processing entry: {}", e),
                    }
                }
            }
            _ => eprintln!("No valid subcommand was used."),
        }
    });
}
