use crate::actions;
use crate::config::Config;
use clap::ArgMatches;
use std::path::PathBuf;

pub fn execute(
    sub_matches: &ArgMatches,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_or_org = sub_matches.get_one::<String>("user-org").expect("required");
    let repo_name = sub_matches.get_one::<String>("repo").expect("required");
    let base_output_dir = sub_matches.get_one::<PathBuf>("basedir").expect("required");
    let output_dir = format!("{}/{}", base_output_dir.display(), user_or_org);
    let new_only = sub_matches.get_flag("new-only");

    println!(
        "Processing single repository: {}/{}",
        user_or_org, repo_name
    );
    actions::process_repositories(
        &[repo_name.to_string()],
        &output_dir,
        user_or_org,
        None,
        new_only,
    )?;
    Ok(())
}
