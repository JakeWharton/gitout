use crate::github::UserReposError::{
	JsonParsingError, MissingResponseData, RequestError, UnsuccessfulResponse,
};
use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;
use thiserror::Error;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/github_schema.graphql",
	query_path = "src/github_repos.graphql",
	response_derives = "Debug"
)]
struct UserRepos;

pub fn user_repos(
	client: &Client,
	user: &str,
	token: &str,
) -> Result<Repositories, UserReposError> {
	let mut owned_after: Option<String> = None;
	let mut owned_repos: Vec<String> = vec![];
	let mut starred_after: Option<String> = None;
	let mut starred_repos: Vec<String> = vec![];
	let mut watched_after: Option<String> = None;
	let mut watched_repos: Vec<String> = vec![];
	loop {
		let query = UserRepos::build_query(user_repos::Variables {
			login: user.to_string(),
			owner_after: owned_after.clone(),
			starred_after: starred_after.clone(),
			watched_after: watched_after.clone(),
		});
		let response = client
			.post("https://api.github.com/graphql")
			.bearer_auth(token)
			.json(&query)
			.send()
			.map_err(RequestError)?;

		if !response.status().is_success() {
			return Err(UnsuccessfulResponse(response.status().to_string()));
		}

		let body: Response<user_repos::ResponseData> = response.json().map_err(JsonParsingError)?;
		let data = body.data.ok_or_else(|| MissingResponseData)?;
		let user = data.user.unwrap();

		let owned_response = user.repositories.edges.unwrap();
		let starred_response = user.starred_repositories.edges.unwrap();
		let watched_response = user.watching.edges.unwrap();
		if owned_response.is_empty() && starred_response.is_empty() && watched_response.is_empty() {
			break;
		}
		for repository in owned_response {
			let repository = repository.unwrap();

			owned_after = Some(repository.cursor);
			owned_repos.push(repository.node.unwrap().name_with_owner);
		}
		for repository in starred_response {
			let repository = repository.unwrap();

			starred_after = Some(repository.cursor);
			starred_repos.push(repository.node.name_with_owner);
		}
		for repository in watched_response {
			let repository = repository.unwrap();

			watched_after = Some(repository.cursor);
			watched_repos.push(repository.node.unwrap().name_with_owner);
		}
	}

	Ok(Repositories {
		owned: owned_repos,
		starred: starred_repos,
		watched: watched_repos,
	})
}

#[derive(Debug, PartialEq)]
pub struct Repositories {
	pub owned: Vec<String>,
	pub starred: Vec<String>,
	pub watched: Vec<String>,
}

#[derive(Error, Debug)]
pub enum UserReposError {
	#[error("Failed to deserialize JSON")]
	JsonParsingError(reqwest::Error),
	#[error("Unsuccessful response: {0}")]
	UnsuccessfulResponse(String),
	#[error("No response data")]
	MissingResponseData,
	#[error(transparent)]
	RequestError(#[from] reqwest::Error),
}

// pub fn archive_repo(client: &Client, token: &str) {
//   let migration_request = MigrationRequest {
//     repositories: vec![],
//   };
//   client.post("https://api.github.com/user/migrations")
//     .bearer_auth(token)
//     .header(ACCEPT, "application/vnd.github.wyandotte-preview+json")
//     .json(&migration_request)
//     .send()
//     .unwrap();
//
//   loop {
//     // TODO read status
//   }
//
//   // TODO download migration
// }
//
// #[derive(Serialize)]
// struct MigrationRequest {
//   repositories: Vec<String>,
// }
