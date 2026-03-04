# Archivum Code Quality Report

## Executive Summary

Archivum is a Rust CLI tool for mirroring GitHub repositories to Gitea/alternative platforms. The codebase shows good structural organization but has several critical bugs, significant code duplication, and inconsistent error handling that need immediate attention.

---

## Critical Bugs (Must Fix)

### 1. Logic Error in `gitea.rs:65-66`
**Location:** `src/gitea.rs:65-66`

**Issue:** `check_user_or_org_exists()` uses `&&` instead of `||`:
```rust
if !self.base_url.contains("localhost") && token.is_empty() {
    return Err(anyhow!("Gitea token is required"));
}
```

**Impact:** This condition will never trigger the error when running locally with an empty token, potentially causing authentication failures.

**Fix:** Change `&&` to `||`:
```rust
if !self.base_url.contains("localhost") || token.is_empty() {
    return Err(anyhow!("Gitea token is required"));
}
```

### 2. Missing CLI Command Registration
**Location:** `src/main.rs`

**Issue:** The `download_repo` module exists (`src/commands/download_repo.rs`) but is NOT registered in the CLI command enum. Users cannot access this functionality.

**Fix:** Add to `src/main.rs`:
```rust
DownloadRepo(DownloadRepoArgs),
// ... in match statement ...
Commands::DownloadRepo(args) => commands::download_repo::run(args),
```

### 3. Silent Error Handling in `mirror.rs:147-160`
**Location:** `src/commands/mirror.rs:147-160`

**Issue:** Organization creation errors are silently ignored:
```rust
let org_id = if let Some(org) = &config.gitea.organization {
    match gitea.create_org(org).await {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Warning: Failed to create org: {}", e);
            // Continues without org_id!
        }
    }
} else {
    0 // Default value used on error
};
```

**Impact:** If org creation fails, subsequent repo creation uses `org_id: 0`, likely failing or creating repos in wrong location.

**Fix:** Return error immediately on org creation failure:
```rust
let org_id = if let Some(org) = &config.gitea.organization {
    gitea.create_org(org).await? // Propagate error
} else {
    0
};
```

---

## Code Quality Issues

### 4. Significant Code Duplication
**Locations:** 
- `src/commands/mirror.rs:162-189`
- `src/commands/upload.rs:85-115`

**Issue:** Nearly identical Gitea organization and repository creation logic appears in both files:
```rust
// mirror.rs
if let Some(org) = &config.gitea.organization {
    let repo_id = gitea.create_repo(&org, &repo_name, &repo.description)
        .await?;
} else {
    let repo_id = gitea.create_repo_for_user(&repo_name, &repo.description)
        .await?;
}

// upload.rs (almost identical)
```

**Impact:** Maintenance burden, inconsistent behavior if fixes needed in one place only.

**Fix:** Extract shared logic into `gitea.rs`:
```rust
// In gitea.rs
pub async fn create_repo_or_org(
    &self,
    config: &GiteaConfig,
    repo_name: &str,
    description: &str,
) -> Result<u64> {
    match &config.organization {
        Some(org) => self.create_repo(org, repo_name, description).await,
        None => self.create_repo_for_user(repo_name, description).await,
    }
}
```

### 5. Inconsistent Error Return Types
**Locations:**
- `src/gitea.rs:38-57` - `create_org()` returns `Result<u64>`
- `src/gitea.rs:59-78` - `create_repo()` returns `Result<u64>`
- `src/gitea.rs:80-97` - `create_org()` (duplicate?) returns `bool`

**Issue:** Some creation methods return `Result<T>` while others return `bool`, forcing callers to handle errors inconsistently.

**Fix:** Standardize all API methods to return `Result<T>` with descriptive errors.

### 6. Missing Error Handling in HTTP Requests
**Location:** `src/gitea.rs:45-50`

**Issue:** Response body not checked:
```rust
let response = self.client.post(url).bearer_auth(token).send().await?;
// No check if response.ok()!
```

**Fix:** Add response validation:
```rust
let response = self.client.post(url).bearer_auth(token).send().await?;
if !response.status().is_success() {
    let error = response.text().await.unwrap_or_default();
    return Err(anyhow!("Gitea API error: {}", error));
}
```

---

## Missing Functionality

### 7. No Test Coverage
**Current State:** Only `config.rs` has unit tests.

**Missing:**
- No tests for GitHub API interactions
- No tests for Gitea API interactions
- No tests for repository processing logic
- No integration tests for CLI commands

**Recommendation:** Add tests for:
- `github.rs`: Test token validation, repo fetching, star checking
- `gitea.rs`: Test org/repo creation, existence checks
- `actions.rs`: Test repo processing, branch handling
- `commands/`: Test CLI argument parsing and command flow

---

## Configuration Security

### 8. Hardcoded Credentials in Repository
**Location:** `config.toml`

**Issue:** Contains actual Gitea credentials:
```toml
token = "gitea_token_here"
base_url = "https://gitea.example.com"
```

**Risk:** Credentials committed to Git repository.

**Fix:**
1. Remove `config.toml` from repository immediately
2. Add to `.gitignore`
3. Update `README.md` with instructions for creating `config.toml`
4. Use `config.toml.example` as template

---

## Code Consistency Issues

### 9. Unused Imports
**Locations:**
- `src/github.rs:13` - `serde::Deserialize` unused
- `src/gitea.rs:1` - `serde::Serialize` unused

**Fix:** Remove unused imports to reduce compilation warnings.

### 10. Inconsistent Naming
**Location:** `src/commands/`

**Issue:** Mixed naming conventions:
- `download.rs` - downloads all starred repos
- `download_repo.rs` - downloads single repo (not registered)
- `mirror.rs` - mirrors all starred repos to Gitea
- `upload.rs` - uploads local repos to Gitea

**Recommendation:** Consider renaming for clarity:
- `download.rs` → `download-starred.rs`
- `download_repo.rs` → `download-single.rs`

---

## Performance Considerations

### 11. Sequential API Calls
**Location:** `src/actions.rs:40-60`

**Issue:** Repositories processed sequentially:
```rust
for repo in repos {
    process_repo(&repo, &config).await?;
}
```

**Impact:** Mirroring 100 repos takes 100x individual API call time.

**Fix:** Use `futures::future::join_all()` for parallel processing:
```rust
let results: Vec<_> = futures::future::join_all(
    repos.iter().map(|r| process_repo(r, config))
).await;
```

---

## Recommendations

### High Priority (Fix Before Next Release)
1. ✅ Fix `&&` → `||` bug in `gitea.rs:65`
2. ✅ Register `download_repo` command in `main.rs`
3. ✅ Fix error propagation in `mirror.rs:147-160`
4. ✅ Remove `config.toml` from repository
5. ✅ Deduplicate Gitea repo creation logic

### Medium Priority (Improve Code Quality)
6. Standardize error return types across all API methods
7. Add response validation for all HTTP requests
8. Add comprehensive test coverage
9. Remove unused imports
10. Implement parallel processing for batch operations

### Low Priority (Enhancements)
11. Consider renaming commands for clarity
12. Add progress bars for long-running operations
13. Add retry logic for failed API calls
14. Implement dry-run mode for mirror/upload commands

---

## Validation Commands

After implementing fixes, run:

```bash
# Fast validation
cargo check

# Linting and formatting
cargo fmt && cargo clippy -- -D warnings

# Tests
cargo test

# Full build
cargo build --release
```

---

## Git Safety Notes

When committing fixes:
- Use conventional commit prefixes (`fix:`, `refactor:`, `test:`)
- Create separate commits for each category of fix
- Test each fix before committing
- Do NOT commit `config.toml` with credentials