use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use reqwest::blocking::Client;

mod args;
mod config;
mod github;

fn main() {
	let args = args::parse_args();
	if args.verbose {
		dbg!(&args);
	}
	let args::Args {
		config,
		destination,
		verbose,
		no_archive,
		dry_run,
	} = args;

	if !dry_run {
		let destination_metadata = fs::metadata(&destination).unwrap();
		if !destination_metadata.is_dir() {
			panic!("Destination must exist and must be a directory")
		}
	}

	let config = fs::read_to_string(config).unwrap();
	let config = config::parse_config(&config).unwrap();
	if verbose {
		dbg!(&config);
	}

	if let Some(github) = config.github {
		static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

		// TODO make this owned by a GitHub struct so it can be entirely encapsulated in github.rs.
		let client = Client::builder()
			.user_agent(APP_USER_AGENT)
			.build()
			.unwrap();

		println!("Querying GitHub information for {0}…", &github.user);
		io::stdout().flush().unwrap();
		let user_repos = github::user_repos(&client, &github.user, &github.token);
		if verbose {
			dbg!(&user_repos);
		}
		println!();

		let mut github_dir = destination.clone();
		github_dir.push("github");

		let mut archive_dir = github_dir.clone();
		archive_dir.push("archive");

		let mut archive_repos = vec![];
		// archive_repos.extend(github.archive.repos.clone());
		if github.archive.owned {
			archive_repos.extend(user_repos.owned.clone());
		}
		let archive_repos: HashSet<String> = archive_repos.into_iter().collect();

		if !no_archive {
			println!("Archiving {0} GitHub repositories…", archive_repos.len());
			for (i, repo) in archive_repos.iter().enumerate() {
				print!(
					"{0}/{1} Archiving {2}… ",
					i + 1,
					&archive_repos.len(),
					&repo
				);
				io::stdout().flush().unwrap();

				github::archive_repo(&client, &archive_dir, &repo, &github.token);

				println!("Done");
			}
			println!();
		}

		if github.clone.gists {
			let mut gists_dir = github_dir.clone();
			gists_dir.push("gists");

			let gist_names = user_repos.gists;
			println!("Checking {0} GitHub gists for updates…", gist_names.len());
			for (i, name) in gist_names.iter().enumerate() {
				print!("\r{0}/{1} ", i + 1, &gist_names.len());
				io::stdout().flush().unwrap();

				let url = format!("https://gist.github.com/{0}.git", &name);
				let username = &github.user;
				let password = &github.token;
				clone_or_fetch_bare(&gists_dir, &name, &url, dry_run, Some((username, password)));
			}
			println!("\n");
		}

		let mut clone_dir = github_dir;
		clone_dir.push("clone");

		let mut clone_repos = vec![];
		clone_repos.extend(user_repos.owned.clone());
		clone_repos.extend(archive_repos);
		clone_repos.extend(github.clone.repos.clone());
		if github.clone.starred {
			clone_repos.extend(user_repos.starred);
		}
		if github.clone.watched {
			clone_repos.extend(user_repos.watched);
		}
		let clone_repos: HashSet<String> = clone_repos.into_iter().collect();

		println!(
			"Checking {0} GitHub repositories for updates…",
			clone_repos.len()
		);
		for (i, repo) in clone_repos.iter().enumerate() {
			print!("\r{0}/{1} ", i + 1, &clone_repos.len());
			io::stdout().flush().unwrap();

			let url = format!("https://github.com/{0}.git", &repo);
			let username = &github.user;
			let password = &github.token;
			clone_or_fetch_bare(&clone_dir, &repo, &url, dry_run, Some((username, password)));
		}
		println!("\n");
	}

	if let Some(git) = config.git {
		let mut git_dir = destination;
		git_dir.push("git");

		println!(
			"Checking {0} git repository clones for updates…",
			git.repos.len()
		);
		for (i, (path, url)) in git.repos.iter().enumerate() {
			print!("\r{0}/{1} ", i + 1, git.repos.len());
			io::stdout().flush().unwrap();

			let url = url.as_str().unwrap();
			clone_or_fetch_bare(&git_dir, &path, url, dry_run, None)
		}
		println!("\n");
	}

	println!("Done!");
}

fn clone_or_fetch_bare(
	dir: &PathBuf,
	repository: &str,
	url: &str,
	dry_run: bool,
	credentials: Option<(&str, &str)>,
) {
	let mut updated = false;

	{
		let mut callbacks = RemoteCallbacks::new();

		if let Some((username, password)) = credentials {
			callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
				Cred::userpass_plaintext(username, password)
			});
		}

		callbacks.transfer_progress(|_progress| {
			if !updated {
				print!("Synchronizing {0} from {1}… ", &repository, &url);
				io::stdout().flush().unwrap();
				updated = true;
			}
			true
		});

		let mut fo = FetchOptions::new();
		fo.remote_callbacks(callbacks);

		if !dry_run {
			let mut repo_dir = dir.clone();
			repo_dir.push(repository);

			let repo_exists = fs::metadata(&repo_dir).map_or_else(|_| false, |m| m.is_dir());

			let repository: Repository;
			let mut origin = if repo_exists {
				repository = Repository::open_bare(&repo_dir).unwrap();
				repository.find_remote("origin").unwrap()
			} else {
				fs::create_dir_all(&repo_dir).unwrap();
				repository = Repository::init_bare(&repo_dir).unwrap();
				repository.remote("origin", url).unwrap()
			};

			origin.fetch(&[] as &[String], Some(&mut fo), None).unwrap();
		}
	}

	if updated {
		println!("Done")
	}
}
