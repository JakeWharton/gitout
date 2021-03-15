use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use std::time::Duration;
use std::{fs, thread};

use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;
use reqwest::header::ACCEPT;
use serde::Deserialize;
use serde::Serialize;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/github_schema.graphql",
	query_path = "src/github_repos.graphql",
	response_derives = "Debug"
)]
struct UserRepos;

pub fn user_repos(client: &Client, user: &str, token: &str) -> Repositories {
	let mut owned_after: Option<String> = None;
	let mut owned_repos: Vec<String> = vec![];
	let mut starred_after: Option<String> = None;
	let mut starred_repos: Vec<String> = vec![];
	let mut watched_after: Option<String> = None;
	let mut watched_repos: Vec<String> = vec![];
	let mut gists_after: Option<String> = None;
	let mut gists_repos: Vec<String> = vec![];
	loop {
		let query = UserRepos::build_query(user_repos::Variables {
			login: user.to_string(),
			owner_after: owned_after.clone(),
			starred_after: starred_after.clone(),
			watched_after: watched_after.clone(),
			gists_after: gists_after.clone(),
		});
		let response = client
			.post("https://api.github.com/graphql")
			.bearer_auth(token)
			.json(&query)
			.send()
			.unwrap();

		let body: Response<user_repos::ResponseData> = response.json().unwrap();
		let user = body.data.unwrap().user.unwrap();

		let owned_response = user.repositories.edges.unwrap();
		let starred_response = user.starred_repositories.edges.unwrap();
		let watched_response = user.watching.edges.unwrap();
		let gists_response = user.gists.edges.unwrap();
		if owned_response.is_empty()
			&& starred_response.is_empty()
			&& watched_response.is_empty()
			&& gists_response.is_empty()
		{
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
		for gist in gists_response {
			let gist = gist.unwrap();

			gists_after = Some(gist.cursor);
			gists_repos.push(gist.node.unwrap().name);
		}
	}

	Repositories {
		owned: owned_repos,
		starred: starred_repos,
		watched: watched_repos,
		gists: gists_repos,
	}
}

#[derive(Debug, PartialEq)]
pub struct Repositories {
	pub owned: Vec<String>,
	pub starred: Vec<String>,
	pub watched: Vec<String>,
	pub gists: Vec<String>,
}

pub fn archive_repo(client: &Client, dir: &PathBuf, repository: &str, token: &str) {
	let migration_request = MigrationRequest {
		repositories: vec![repository.to_owned()],
	};
	let create_response: MigrationResponse = client
		.post("https://api.github.com/user/migrations")
		.bearer_auth(token)
		.header(ACCEPT, "application/vnd.github.wyandotte-preview+json")
		.json(&migration_request)
		.send()
		.unwrap()
		.error_for_status()
		.unwrap()
		.json()
		.unwrap();
	let migration_id = create_response.id;
	let mut migration_state = create_response.state;

	let mut wait = Duration::from_secs(2);
	loop {
		if migration_state == "exported" {
			break;
		}
		if migration_state == "failed" {
			panic!("Creating migration for {} failed", &repository);
		}

		thread::sleep(wait);
		if wait < Duration::from_secs(64) {
			wait *= 2
		}

		let status_url = format!("https://api.github.com/user/migrations/{0}", migration_id);
		let status_response: MigrationResponse = client
			.get(&status_url)
			.bearer_auth(token)
			.header(ACCEPT, "application/vnd.github.wyandotte-preview+json")
			.send()
			.unwrap()
			.error_for_status()
			.unwrap()
			.json()
			.unwrap();
		migration_state = status_response.state;
	}

	// In order to never lose data if we crash we must perform a dance to update archives:
	// 1. Download the new archive to repo.zip.new.
	// 2. Delete the old archive repo.zip.
	// 3. Rename the new archive from repo.zip.new to repo.zip.

	let mut archive_old = dir.clone();
	archive_old.push(format!("{0}.zip", &repository));
	let mut archive_new = dir.clone();
	archive_new.push(format!("{0}.zip.new", &repository));

	let mut archive_dir = archive_old.clone();
	archive_dir.pop();
	if !fs::metadata(&archive_dir).map_or_else(|_| false, |m| m.is_dir()) {
		fs::create_dir_all(&archive_dir).unwrap();
	}

	let archive_old_exists = fs::metadata(&archive_old).map_or_else(|_| false, |m| m.is_file());
	let archive_new_exists = fs::metadata(&archive_new).map_or_else(|_| false, |m| m.is_file());

	if archive_new_exists {
		fs::remove_file(&archive_new).unwrap();
	}

	// Step 1:
	let download_url = format!(
		"https://api.github.com/user/migrations/{0}/archive",
		migration_id
	);
	let mut download_request = client
		.get(&download_url)
		.bearer_auth(token)
		.header(ACCEPT, "application/vnd.github.wyandotte-preview+json")
		.send()
		.unwrap()
		.error_for_status()
		.unwrap();

	let mut archive_file = File::create(&archive_new).unwrap();
	copy(&mut download_request, &mut archive_file).unwrap();

	// Step 2:
	if archive_old_exists {
		fs::rename(&archive_old, &archive_old).unwrap();
	}

	// Step 3:
	fs::rename(&archive_new, &archive_old).unwrap();
}

#[derive(Serialize)]
struct MigrationRequest {
	repositories: Vec<String>,
}

#[derive(Deserialize)]
struct MigrationResponse {
	id: u64,
	state: String,
}
