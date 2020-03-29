use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "src/github_schema.graphql",
  query_path = "src/github_repos.graphql",
  response_derives = "Debug"
)]
struct UserRepos;

pub fn user_repos(user: &str, token: &str) -> Vec<String> {
  static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
  );

  let client = Client::builder()
    .user_agent(APP_USER_AGENT)
    .build()
    .unwrap();

  let mut repos: Vec<String> = vec![];
  let mut after: Option<String> = None;
  loop {
    let query = UserRepos::build_query(user_repos::Variables {
      login: user.to_string(),
      after: after.clone(),
    });
    let response = client
      .post("https://api.github.com/graphql")
      .bearer_auth(token)
      .json(&query)
      .send()
      .unwrap();

    let body: Response<user_repos::ResponseData> = response.json().unwrap();
    let edges = body.data.unwrap().user.unwrap().repositories.edges.unwrap();
    if edges.len() == 0 {
      break;
    }
    for edge in edges {
      let edge = edge.unwrap();

      after = Some(edge.cursor);
      repos.push(edge.node.unwrap().name_with_owner);
    }
  }

  return repos;
}
