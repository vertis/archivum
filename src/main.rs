mod commands;
use clap::{arg, command, value_parser};
use reqwest::Client;
use std::path::PathBuf;
use duct::cmd;
use crate::commands::{get_repositories, get_starred_repositories};

async fn create_org(api_url: &str, token: &str, org_name: &str) -> bool {
    let client = Client::new();
    let new_org = serde_json::json!({
        "username": org_name,
        "full_name": format!("{} Full Name", org_name),
        "description": format!("{} is a great organization.", org_name),
        "website": "",
        "location": "World",
        "visibility": "private",
    });

    let body = serde_json::to_string(&new_org).unwrap();

    let res = client
        .post(format!("{}/api/v1/orgs", api_url))
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;

    match res {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

async fn check_org_exists(api_url: &str, token: &str, org_name: &str) -> bool {
    let client = Client::new();
    let res = client
        .get(format!("{}/api/v1/orgs/{}", api_url, org_name))
        .bearer_auth(token)
        .send()
        .await;

    matches!(res, Ok(response) if response.status().is_success())
}

async fn check_repo_exists(api_url: &str, token: &str, org_name: &str, repo_name: &str) -> bool {
    let client = Client::new();
    let res = client
        .get(format!(
            "{}/api/v1/repos/{}/{}",
            api_url, org_name, repo_name
        ))
        .bearer_auth(token)
        .send()
        .await;

    matches!(res, Ok(response) if response.status().is_success())
}

async fn create_repo(api_url: &str, token: &str, org_name: &str, repo_name: &str) -> bool {
    let client = Client::new();
    let new_repo = serde_json::json!({
        "name": repo_name,
        "description": format!("{} is a great repository.", repo_name),
        "private": true,
    });

    let body = serde_json::to_string(&new_repo).unwrap();

    let res = client
        .post(format!("{}/api/v1/orgs/{}/repos", api_url, org_name))
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;

    match res {
        Ok(response) => {
            let status_success = response.status().is_success();
            if let Ok(text) = response.text().await {
                eprintln!(
                    "Trying to create repository {}/{}. Response body: {}",
                    org_name, repo_name, text
                );
            }
            status_success
        }
        Err(e) => {
            // if let Ok(text) = e..text().await {
            //     eprintln!(
            //         "Failed to create repository {}/{}. Response body: {}",
            //         org_name, repo_name, text
            //     );
            // }
            eprintln!(
                "Failed to create repository {}/{}. Response body: {}",
                org_name,
                repo_name,
                e.to_string()
            );
            false
        }
    }
}

async fn mirror_push_repo(
    repo_path: &PathBuf,
    api_url: &str,
    org_name: &str,
    repo_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo_url = format!("{}/{}/{}", api_url, org_name, repo_name);
    let authenticated_repo_url = format!("http://vertis:4rch1v1st@{}", &repo_url[7..]);
    let _output = cmd!("git", "push", "--mirror", &authenticated_repo_url)
        .dir(repo_path)
        .run()?;
    Ok(())
}

async fn check_user_or_org_exists(
    api_url: &str,
    token: &str,
    name: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = Client::new();
    let user_res = client
        .get(format!("{}/api/v1/users/{}", api_url, name))
        .bearer_auth(token)
        .send()
        .await;

    let org_res = client
        .get(format!("{}/api/v1/orgs/{}", api_url, name))
        .bearer_auth(token)
        .send()
        .await;

    if user_res.is_ok() && user_res.unwrap().status().is_success() {
        return Ok(true); // User exists
    }

    if org_res.is_ok() && org_res.unwrap().status().is_success() {
        return Ok(true); // Org exists
    }

    Ok(false) // Neither user nor org exists
}

async fn create_org_if_no_conflict(
    api_url: &str,
    token: &str,
    org_name: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    if check_user_or_org_exists(api_url, token, org_name).await? {
        Err("A user or organization with the same name already exists.".into())
    } else {
        Ok(create_org(api_url, token, org_name).await)
    }
}

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
            _ => eprintln!("No valid subcommand was used."),
        }
    });
}
