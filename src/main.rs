mod actions;
mod commands;
mod config;
mod git;
mod gitea;
mod github;

use clap::{Arg, ArgMatches, Command};

fn main() {
    let matches = Command::new("archivum")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Mirrors GitHub repositories for a specified user or organization")
        .subcommand(
            Command::new("mirror")
                .about("Mirrors repositories based on the configuration file")
                .arg(
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG_FILE")
                        .help("Specifies the path to the configuration file")
                        .default_value("config.toml"),
                )
                .arg(
                    Arg::new("new-only")
                        .long("new-only")
                        .help("Only download repositories that don't exist locally")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("download")
                .about("Downloads repositories based on the configuration file")
                .arg(
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG_FILE")
                        .help("Specifies the path to the configuration file")
                        .default_value("config.toml"),
                )
                .arg(
                    Arg::new("new-only")
                        .long("new-only")
                        .help("Only download repositories that don't exist locally")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("upload")
                .about("Uploads repositories based on the configuration file")
                .arg(
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG_FILE")
                        .help("Specifies the path to the configuration file")
                        .default_value("config.toml"),
                ),
        )
        .subcommand(
            Command::new("download_repo")
                .about("Downloads a single repository")
                .arg(
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG_FILE")
                        .help("Specifies the path to the configuration file")
                        .default_value("config.toml"),
                )
                .arg(
                    Arg::new("user-org")
                        .help("User or organization name")
                        .required(true),
                )
                .arg(Arg::new("repo").help("Repository name").required(true))
                .arg(
                    Arg::new("basedir")
                        .help("Base output directory")
                        .required(true),
                )
                .arg(
                    Arg::new("new-only")
                        .long("new-only")
                        .help("Only download repositories that don't exist locally")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("mirror", sub_matches)) => execute_command(sub_matches, commands::mirror::execute),
        Some(("download", sub_matches)) => {
            execute_command(sub_matches, commands::download::execute)
        }
        Some(("upload", sub_matches)) => execute_command(sub_matches, commands::upload::execute),
        Some(("download_repo", sub_matches)) => {
            execute_command(sub_matches, commands::download_repo::execute)
        }
        _ => {
            eprintln!("No valid subcommand was used. Use 'archivum mirror', 'archivum download', 'archivum upload', or 'archivum download_repo' to run the commands.");
            std::process::exit(1);
        }
    }
}

fn execute_command<F>(sub_matches: &ArgMatches, command: F)
where
    F: Fn(&clap::ArgMatches, &config::Config) -> Result<(), Box<dyn std::error::Error>>,
{
    let config_path = sub_matches.get_one::<String>("config").expect("required");
    match config::Config::from_file(config_path) {
        Ok(config) => {
            if let Err(e) = command(sub_matches, &config) {
                eprintln!("Error executing command: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error reading configuration file: {}", e);
            eprintln!(
                "Make sure the file '{}' exists and is properly formatted.",
                config_path
            );
            std::process::exit(1);
        }
    }
}
