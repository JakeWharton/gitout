use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "src/github_schema.graphql",
  query_path = "src/github_repos.graphql",
  response_derives = "Debug"
)]
struct UserRepos;

pub fn user_repos(user: &str, token: &str) -> Repositories {
  static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
  );

  let client = Client::builder()
    .user_agent(APP_USER_AGENT)
    .build()
    .unwrap();

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
      .unwrap();

    let body: Response<user_repos::ResponseData> = response.json().unwrap();
    let user = body.data.unwrap().user.unwrap();

    let owned_response = user.repositories.edges.unwrap();
    let starred_response = user.starred_repositories.edges.unwrap();
    let watched_response = user.watching.edges.unwrap();
    if owned_response.len() == 0 && starred_response.len() == 0 && watched_response.len() == 0 {
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

  return Repositories {
    owned: owned_repos,
    starred: starred_repos,
    watched: watched_repos,
  };
}

pub struct Repositories {
  pub owned: Vec<String>,
  pub starred: Vec<String>,
  pub watched: Vec<String>,
}
