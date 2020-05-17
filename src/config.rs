use serde::Deserialize;
use toml::de::Error;
use toml::value::Table;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
	pub version: u32,
	pub github: Option<GitHub>,
	pub git: Option<Git>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GitHub {
	pub user: String,
	pub token: String,
	// #[serde(default)]
	// pub archive: GitHubArchive,
	#[serde(default)]
	pub clone: GitHubClone,
}

// #[derive(Debug, Deserialize, PartialEq)]
// pub struct GitHubArchive {
//   #[serde(default)]
//   pub owned: bool,
//   #[serde(default)]
//   pub repos: Vec<String>,
// }
//
// impl Default for GitHubArchive {
//   fn default() -> Self {
//     GitHubArchive {
//       owned: true,
//       repos: vec![],
//     }
//   }
// }

#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct GitHubClone {
	#[serde(default)]
	pub starred: bool,
	#[serde(default)]
	pub watched: bool,
	#[serde(default)]
	pub repos: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Git {
	#[serde(default)]
	pub repos: Table,
}

pub fn parse_config(s: &str) -> Result<Config, Error> {
	toml::from_str(s)
}

#[cfg(test)]
mod test {
	use super::*;
	use toml::value::Value;

	#[test]
	fn empty() {
		let actual = parse_config(
			r#"
      version = 0
      "#,
		)
		.unwrap();
		let expected = Config {
			version: 0,
			github: None,
			git: None,
		};
		assert_eq!(actual, expected)
	}

	#[test]
	fn full() {
		let actual = parse_config(
			r#"
      version = 0

      [github]
      user = "user"
      token = "token"

      [github.clone]
      starred = true
      watched = true
      repos = [
        "example/two",
      ]

      [git.repos]
      example = "https://example.com/example.git"
      "#,
		)
		.unwrap();
		let mut repos = Table::new();
		repos.insert(
			"example".to_string(),
			Value::from("https://example.com/example.git"),
		);
		let expected = Config {
			version: 0,
			github: Some(GitHub {
				user: "user".to_string(),
				token: "token".to_string(),
				// archive: GitHubArchive { owned: false, repos: vec!["example/one".to_string()] },
				clone: GitHubClone {
					starred: true,
					watched: true,
					repos: vec!["example/two".to_string()],
				},
			}),
			git: Some(Git { repos }),
		};
		assert_eq!(actual, expected)
	}
}
