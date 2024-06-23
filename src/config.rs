use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    pub users: Vec<String>,
    pub organizations: Vec<String>,
    pub repositories: Vec<String>,
    pub output_dir: String,
    pub gitea: Option<GiteaConfig>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct GiteaConfig {
    pub url: String,
    pub token: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_from_file() {
        let config_content = r#"
            users = ["user1", "user2"]
            organizations = ["org1", "org2"]
            repositories = ["repo1", "repo2"]
            output_dir = "/tmp/output"

            [gitea]
            api_url = "https://gitea.example.com"
            token = "abcdef123456"
            username = "testuser"
            password = "testpassword"
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", config_content).unwrap();

        let config = Config::from_file(temp_file.path()).unwrap();

        assert_eq!(config.users, vec!["user1", "user2"]);
        assert_eq!(config.organizations, vec!["org1", "org2"]);
        assert_eq!(config.repositories, vec!["repo1", "repo2"]);
        assert_eq!(config.output_dir, "/tmp/output");
        assert_eq!(config.gitea, Some(GiteaConfig {
            url: "https://gitea.example.com".to_string(),
            token: "abcdef123456".to_string(),
            username: "testuser".to_string(),
            password: "testpassword".to_string(),
        }));
    }

    #[test]
    fn test_config_from_file_without_gitea() {
        let config_content = r#"
            users = ["user1", "user2"]
            organizations = ["org1", "org2"]
            repositories = ["repo1", "repo2"]
            output_dir = "/tmp/output"
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", config_content).unwrap();

        let config = Config::from_file(temp_file.path()).unwrap();

        assert_eq!(config.users, vec!["user1", "user2"]);
        assert_eq!(config.organizations, vec!["org1", "org2"]);
        assert_eq!(config.repositories, vec!["repo1", "repo2"]);
        assert_eq!(config.output_dir, "/tmp/output");
        assert_eq!(config.gitea, None);
    }

    #[test]
    fn test_config_from_file_invalid_toml() {
        let invalid_config_content = r#"
            users = ["user1", "user2"]
            organizations = ["org1", "org2"]
            repositories = ["repo1", "repo2"]
            output_dir = /tmp/output  # Missing quotes around string
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", invalid_config_content).unwrap();

        let result = Config::from_file(temp_file.path());
        assert!(result.is_err());
    }
}
