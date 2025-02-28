# Archivum Commands

Archivum is a tool for mirroring GitHub repositories. Below are all available commands.

## Core Commands

### `mirror`

Mirrors GitHub repositories based on the configuration file. If `include_starred = true` in the config file, it will also mirror your starred repositories.

**Options:**
- `-c, --config <CONFIG_FILE>`: Specifies the path to the configuration file (default: `config.toml`)
- `--new-only`: Only download repositories that don't exist locally

**Usage Examples:**
```bash
archivum mirror
archivum mirror --config custom-config.toml
archivum mirror --new-only
```


### `download`

Downloads GitHub repositories based on the configuration file without pushing to Gitea. If `include_starred = true` in the config file, it will also download your starred repositories.

**Options:**
- `-c, --config <CONFIG_FILE>`: Specifies the path to the configuration file (default: `config.toml`)
- `--new-only`: Only download repositories that don't exist locally

**Usage Examples:**
```bash
archivum download
archivum download --config custom-config.toml
archivum download --new-only
```


### `upload`

Uploads repositories to a Gitea instance based on the configuration file.

**Options:**
- `-c, --config <CONFIG_FILE>`: Specifies the path to the configuration file (default: `config.toml`)

**Usage Examples:**
```bash
archivum upload
archivum upload --config custom-config.toml
```

## Configuration

Archivum requires a configuration file in TOML format. Example `config.toml`:

```toml
# List of GitHub users to mirror
users = ["user1", "user2"]

# List of GitHub organizations to mirror
organizations = ["org1", "org2"]

# List of specific repositories to mirror (in the format "owner/repo")
repositories = ["owner1/repo1", "owner2/repo2"]

# Whether to include your starred repositories (true/false)
include_starred = false

# Output directory for mirrored repositories
output_dir = "/path/to/output/directory"

# Gitea configuration (optional, required for upload and mirror commands)
[gitea]
url = "https://gitea.example.com"
token = "your_gitea_api_token"
username = "your_gitea_username"
password = "your_gitea_password"
```

## Requirements

- GitHub CLI (`gh`) must be installed and authenticated
- Git with LFS support