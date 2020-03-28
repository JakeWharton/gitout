use std::fs;
use std::path::PathBuf;

use git2::build::RepoBuilder;
use git2::Repository;

mod args;
mod config;

fn main() {
  let args = args::parse_args();
  if args.verbose {
    dbg!(&args);
  }
  let args::Args {
    config,
    destination,
    verbose,
  } = args;

  let destination_metadata = fs::metadata(&destination).unwrap();
  if !destination_metadata.is_dir() {
    panic!("Destination must exist and must be a directory")
  }

  let config = fs::read_to_string(config).unwrap();
  let config = config::parse_config(&config).unwrap();
  if verbose {
    dbg!(&config);
  }

  if let Some(github) = config.github {
    let mut github_dir = destination.clone();
    github_dir.push("github");

    for repo in github.clone.repos {
      // TODO switch to https and authenticated clones
      let url = format!("git://github.com/{0}.git", &repo);
      let path = format!("{0}/git", &repo);
      clone_or_fetch_bare(&github_dir, &path, &url);
    }
  }

  if let Some(git) = config.git {
    let mut git_dir = destination.clone();
    git_dir.push("git");

    for (path, url) in git.repos {
      let url = url.as_str().unwrap();
      clone_or_fetch_bare(&git_dir, &path, url)
    }
  }
}

fn clone_or_fetch_bare(dir: &PathBuf, path: &str, url: &str) {
  let mut repo_dir = dir.clone();
  repo_dir.push(path);

  if fs::metadata(&repo_dir).map_or_else(|_| false, |m| m.is_dir()) {
    let repository = Repository::open_bare(&repo_dir).unwrap();
    let mut origin = repository.find_remote("origin").unwrap();
    origin.fetch(&[] as &[String], None, None).unwrap();
  } else {
    fs::create_dir_all(&repo_dir).unwrap();

    let mut repo_builder = RepoBuilder::new();
    repo_builder.bare(true);
    repo_builder.clone(url, &repo_dir).unwrap();
  }
}
