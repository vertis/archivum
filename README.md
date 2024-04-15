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

### Download Repositories

To download all repositories for a user or organization:

```bash
cargo run -- download -u <USER_OR_ORG> -b <BASE_OUTPUT_DIR>
```

### Download Starred Repositories

To download all starred repositories:

```bash
cargo run -- download-starred -b <BASE_OUTPUT_DIR>
```

### Upload Repositories

To upload all repositories from a local directory to a destination:

```bash
cargo run -- upload -d <DESTINATION> -p <PATH>
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
