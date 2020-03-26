use serde::Deserialize;
use toml::de::Error;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
  pub version: u32,
  pub api_key: String,
  #[serde(default)]
  pub github: GitHub,
  #[serde(default)]
  pub git: Git,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct GitHub {
  #[serde(default)]
  pub archive: GitHubArchive,
  #[serde(default)]
  pub clone: GitHubClone,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GitHubArchive {
  #[serde(default)]
  pub owned: bool,
  #[serde(default)]
  pub repos: Vec<String>,
}

impl Default for GitHubArchive {
  fn default() -> Self {
    GitHubArchive {
      owned: true,
      repos: vec![]
    }
  }
}

#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct GitHubClone {
  #[serde(default)]
  pub starred: bool,
  #[serde(default)]
  pub watched: bool,
  #[serde(default)]
  pub repos: Vec<String>,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct Git {
  #[serde(default)]
  pub clone: GitCLone,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct GitCLone {
  #[serde(default)]
  pub repos: Vec<String>,
}

pub fn parse_config(s: &str) -> Result<Config, Error> {
  toml::from_str(s)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn empty() {
    let actual = parse_config(r#"
      version = 0
      api_key = "key"
    "#).unwrap();
    let expected = Config {
      version: 0,
      api_key: "key".to_string(),
      github: GitHub {
        archive: GitHubArchive { owned: true, repos: vec![] },
        clone: GitHubClone { starred: false, watched: false, repos: vec![] },
      },
      git: Git {
        clone: GitCLone { repos: vec![] },
      },
    };
    assert_eq!(actual, expected)
  }

  #[test]
  fn full() {
    let actual = parse_config(r#"
      version = 0
      api_key = "key"

      [github.archive]
      owned = false
      repos = [
        "example/one",
      ]

      [github.clone]
      starred = true
      watched = true
      repos = [
        "example/two",
      ]

      [git.clone]
      repos = [
        "https://example.com/example.git",
      ]

    "#).unwrap();
    let expected = Config {
      version: 0,
      api_key: "key".to_string(),
      github: GitHub {
        archive: GitHubArchive { owned: false, repos: vec!["example/one".to_string()] },
        clone: GitHubClone { starred: true, watched: true, repos: vec!["example/two".to_string()] },
      },
      git: Git {
        clone: GitCLone { repos: vec!["https://example.com/example.git".to_string()] },
      },
    };
    assert_eq!(actual, expected)
  }
}
