use duct::cmd;
use std::path::Path;

fn main() {
    let user_or_org = "vertis"; // Replace with the actual user or org name
    let base_output_dir = "mirror"; // Replace with the actual base output directory path
    let output_dir = format!("{}/{}", base_output_dir, user_or_org); // Construct the full output directory path

    // Get a list of all repositories under the given user or org
    let repos = match cmd!("gh", "repo", "list", user_or_org, "--json", "name").read() {
        Ok(output) => serde_json::from_str::<Vec<serde_json::Value>>(&output)
            .expect("Failed to parse JSON")
            .into_iter()
            .map(|repo| {
                repo["name"]
                    .as_str()
                    .expect("Expected a string")
                    .to_string()
            })
            .collect::<Vec<String>>(),
        Err(e) => {
            eprintln!("Failed to list repositories: {}", e);
            return;
        }
    };

    for repo in repos {
        let repo_path = format!("{}/{}.git", output_dir, repo);
        if Path::new(&repo_path).exists() {
            // Update the clone if it already exists
            if let Err(e) = cmd!("git", "-C", &repo_path, "remote", "update").run() {
                eprintln!("Failed to update repository {}: {}", repo, e);
            }
        } else {
            // Bare mirror clone the repository if it doesn't exist
            if let Err(e) = cmd!(
                "git",
                "clone",
                "--mirror",
                &format!("https://github.com/{}/{}.git", user_or_org, repo),
                &repo_path
            )
            .run()
            {
                eprintln!("Failed to clone repository {}: {}", repo, e);
            }
        }
    }
}
