use crate::actions;
use crate::github::get_repositories;
use clap::ArgMatches;
use std::path::PathBuf;

pub fn execute(sub_matches: &ArgMatches) {
    let user_or_org = sub_matches.get_one::<String>("user-org").expect("required");
    let base_output_dir = sub_matches.get_one::<PathBuf>("basedir").expect("required");
    let output_dir = format!("{}/{}", base_output_dir.display(), user_or_org);

    let repos = match get_repositories(user_or_org) {
        Ok(repos) => repos,
        Err(e) => {
            eprintln!("Failed to list repositories: {}", e);
            std::process::exit(1);
        }
    };

    actions::process_repositories(&repos, &output_dir, user_or_org, None);
}
