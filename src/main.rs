mod actions;
mod commands;
mod config;
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
                ),
        )
        .subcommand(
            Command::new("mirror-starred")
                .about("Mirrors starred repositories based on the configuration file")
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
            Command::new("download")
                .about("Downloads repositories based on the configuration file")
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
            Command::new("download-starred")
                .about("Downloads starred repositories based on the configuration file")
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
        .get_matches();

    match matches.subcommand() {
        Some(("mirror", sub_matches)) => execute_command(sub_matches, commands::mirror::execute),
        Some(("mirror-starred", sub_matches)) => {
            execute_command(sub_matches, commands::mirror_starred::execute)
        }
        Some(("download", sub_matches)) => {
            execute_command(sub_matches, commands::download::execute)
        }
        Some(("download-starred", sub_matches)) => {
            execute_command(sub_matches, commands::download_starred::execute)
        }
        Some(("upload", sub_matches)) => execute_command(sub_matches, commands::upload::execute),
        _ => {
            eprintln!("No valid subcommand was used. Use 'archivum mirror', 'archivum mirror-starred', 'archivum download', 'archivum download-starred', or 'archivum upload' to run the commands.");
            std::process::exit(1);
        }
    }
}

fn execute_command<F>(sub_matches: &ArgMatches, command: F)
where
    F: Fn(&config::Config) -> Result<(), Box<dyn std::error::Error>>,
{
    let config_path = sub_matches.get_one::<String>("config").expect("required");
    match config::Config::from_file(config_path) {
        Ok(config) => {
            if let Err(e) = command(&config) {
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
