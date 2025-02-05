// Update a repository by fetching the latest changes.
pub fn update_repo_with_lfs(
    repo_path: &str,
    _repo: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    cmd!("git", "--git-dir", repo_path, "fetch", "--all").run()?;

    // Handle LFS objects after fetching changes
    cmd!("git", "lfs", "fetch", "--all", repo_path).run()?;

    Ok(())
}
