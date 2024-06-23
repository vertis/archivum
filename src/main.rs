mod actions;
mod commands;
mod gitea;
mod github;
use crate::gitea::{
    check_repo_exists, create_org_if_no_conflict, create_repo,
};
use clap::{arg, command, value_parser};
use std::path::PathBuf;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let matches = command!("archivum")
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
                crate::commands::download::execute(sub_matches);
            }
            Some(("download-repo", sub_matches)) => {
                crate::commands::download_repo::execute(sub_matches);
            },
            Some(("download-starred", sub_matches)) => {
                crate::commands::download_starred::execute(sub_matches);
            },
            Some(("upload", sub_matches)) => {
                crate::commands::upload::execute(sub_matches).await;
            }
            _ => eprintln!("No valid subcommand was used."),
        }
    });
}
