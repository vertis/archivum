# Archivum

**Alpha Software Warning**: Please note that Archivum is currently in alpha. It may contain bugs and incomplete features. We welcome contributions and feedback to improve it.

Archivum is a Rust application designed to mirror GitHub repositories for specified users or organizations. It supports operations like downloading, uploading, and managing repositories. It currently supports downloading from GitHub and uploading to Gitea.

[**Archivum** (_Latin_)](https://en.wiktionary.org/wiki/archivum): public records office; archives; archive room.

## Motivation

As a developer and open-source enthusiast, I deeply value the wealth of knowledge and collaborative spirit found within the GitHub community. However, I've witnessed instances where valuable repositories and user accounts have been taken down due to DMCA claims or other legal issues, often without proper consideration for the public interest.

This experience, coupled with the inspiring work of the Long Now Foundation, has motivated me to create Archivum. The Long Now Foundation promotes long-term thinking and responsibility, encouraging society to consider the bigger picture and the long-term consequences of our actions. In the spirit of their mission, Archivum is designed to be a durable solution for mirroring and preserving GitHub repositories.

By making it easy to create backups of important projects and contributions, Archivum aims to ensure that this knowledge remains accessible and resilient in the face of potential censorship or erasure. I believe that the open-source community thrives on the free exchange of ideas and information, and Archivum is one of my contributions to this cause.

Through features like automated mirroring and the ability to upload to alternative platforms, Archivum is designed to be a reliable and long-lasting solution for preserving the history and progress of software development. It's my hope that this tool will serve the community by promoting the longevity and accessibility of open-source projects for years to come, embodying the principles championed by the Long Now Foundation.

## Features

- **Download Repositories**: Download all repositories for a specified user or organization.
- **Download Starred Repositories**: Download all starred repositories of the authenticated user.
- **Upload Repositories**: Upload mirrored repositories to a specified destination.
- **Repository Management**: Automatically create organizations and repositories if they do not exist at the destination.

## Requirements

- Rust
- Cargo
- Git
- GitHub CLI

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/vertis/archivum.git
cd archivum
cargo build --release
```

## Usage

Archivum supports three main commands: `mirror`, `mirror-starred`, and `download`. Each command can be run with an optional configuration file. If no configuration file is specified, it will use the default `config.toml` in the current directory.

### Mirror Repositories

To mirror repositories based on the configuration file:

```bash
cargo run -- mirror [-c <CONFIG_FILE>]
```

### Mirror Starred Repositories

To mirror starred repositories based on the configuration file:

```bash
cargo run -- mirror-starred [-c <CONFIG_FILE>]
```

### Download Starred Repositories

To download starred repositories based on the configuration file:

```bash
cargo run -- download-starred [-c <CONFIG_FILE>]
```

### Download Repositories

To download repositories based on the configuration file:

```bash
cargo run -- download [-c <CONFIG_FILE>]
```

### Upload Repositories

To upload repositories based on the configuration file:

```bash
cargo run -- upload [-c <CONFIG_FILE>]
```

### Configuration File

The configuration file (default: `config.toml`) should contain the necessary settings for the GitHub source and the destination (e.g., Gitea). Make sure to set up this file correctly before running any commands.

Example `config.toml`:

```toml
# List of GitHub users to mirror
users = ["user1", "user2"]

# List of GitHub organizations to mirror
organizations = ["org1", "org2"]

# List of specific repositories to mirror (in the format "owner/repo")
repositories = ["owner1/repo1", "owner2/repo2"]

# Output directory for mirrored repositories
output_dir = "/path/to/output/directory"

# Gitea configuration (optional)
[gitea]
url = "https://gitea.example.com"
token = "your_gitea_api_token"
username = "your_gitea_username"
password = "your_gitea_password"
```

## Configuration

Ensure that your `.gitignore` is set up to ignore the appropriate directories and files:

```plaintext
/target
mirror
```

## Contributing

Contributions are welcome! Please fork the repository and open a pull request with your changes.

## License

This project is licensed under the terms of the MIT license. For more details, please see the [LICENSE](LICENSE) file.
