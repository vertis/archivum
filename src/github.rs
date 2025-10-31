use duct::cmd;

pub fn get_starred_repositories() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = cmd!(
        "gh",
        "api",
        "/user/starred?per_page=100",
        "-q",
        ".[].full_name"
    )
    .read()?;
    let repos = output
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    Ok(repos)
}

pub fn get_repositories(user_or_org: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = cmd!(
        "gh",
        "api",
        &format!("users/{}/repos", user_or_org),
        "--paginate",
        "-q",
        ".[].name"
    )
    .read()?;

    let repos = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    Ok(repos)
}

// clone with --mirror and lfs, we should probably make this more generic at some point
pub fn clone_with_mirror(
    user_or_org: &str,
    repo: &str,
    repo_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    cmd!(
        "git",
        "clone",
        "--mirror",
        &format!("https://github.com/{}/{}.git", user_or_org, repo),
        repo_path
    )
    .run()?;

    // Initialize and fetch LFS objects after cloning
    cmd!("git", "--git-dir", repo_path, "lfs", "fetch", "--all").run()?;

    Ok(())
}
