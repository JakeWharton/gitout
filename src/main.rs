use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use git2::build::RepoBuilder;
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
    static APP_USER_AGENT: &str = concat!(
      env!("CARGO_PKG_NAME"),
      "/",
      env!("CARGO_PKG_VERSION"),
    );

    let client = Client::builder()
      .user_agent(APP_USER_AGENT)
      .build()
      .unwrap();

    let user_repos = github::user_repos(&client, &github.user, &github.token);
    if verbose {
      dbg!(&user_repos);
    }

    let mut github_dir = destination.clone();
    github_dir.push("github");

    // let mut archive_dir = github_dir.clone();
    // archive_dir.push("archive");
    //
    // let mut archive_repos = github.archive.repos.clone();
    // if github.archive.owned {
    //   archive_repos.extend(user_repos.owned.clone());
    // }
    // let archive_repos: HashSet<String> = archive_repos.into_iter().collect();
    // for repo in &archive_repos {
    //   // TODO archive
    // }

    let mut clone_dir = github_dir;
    clone_dir.push("clone");

    let mut clone_repos = github.clone.repos.clone();
    // clone_repos.extend(archive_repos);
    clone_repos.extend(user_repos.owned.clone());
    if github.clone.starred {
      clone_repos.extend(user_repos.starred);
    }
    if github.clone.watched {
      clone_repos.extend(user_repos.watched);
    }
    let clone_repos: HashSet<String> = clone_repos.into_iter().collect();
    for repo in &clone_repos {
      let url = format!("https://github.com/{0}.git", &repo);

      let mut callbacks = RemoteCallbacks::new();
      let username = &github.user;
      let password = &github.token;
      callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        Cred::userpass_plaintext(username, password)
      });

      let mut fo = FetchOptions::new();
      fo.remote_callbacks(callbacks);

      clone_or_fetch_bare(&clone_dir, &repo, &url, Some(fo), dry_run);
    }
  }

  if let Some(git) = config.git {
    let mut git_dir = destination;
    git_dir.push("git");

    for (path, url) in git.repos {
      let url = url.as_str().unwrap();
      clone_or_fetch_bare(&git_dir, &path, url, None, dry_run)
    }
  }
}

fn clone_or_fetch_bare(dir: &PathBuf, path: &str, url: &str, mut fo: Option<FetchOptions>, dry_run: bool) {
  let mut repo_dir = dir.clone();
  repo_dir.push(path);

  if fs::metadata(&repo_dir).map_or_else(|_| false, |m| m.is_dir()) {
    println!("Fetching {0}", &path);

    if !dry_run {
      let repository = Repository::open_bare(&repo_dir).unwrap();
      let mut origin = repository.find_remote("origin").unwrap();
      origin.fetch(&[] as &[String], fo.as_mut(), None).unwrap();
    }
  } else {
    println!("Cloning {0} from {1}", &path, &url);

    if !dry_run {
      fs::create_dir_all(&repo_dir).unwrap();

      let mut repo_builder = RepoBuilder::new();
      if let Some(fo) = fo {
        repo_builder.fetch_options(fo);
      }
      repo_builder.bare(true);
      repo_builder.clone(url, &repo_dir).unwrap();
    }
  }
}
