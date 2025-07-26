pub mod project;

use reqwest::Client;

pub use self::project::Project;

const ENDPOINT: &str = "https://api.modrinth.com/v2";

pub async fn fetch_project(project_id: &str) -> Result<Project, reqwest::Error> {
  let url = format!("{ENDPOINT}/project/{project_id}");
  let client = Client::new();
  let resp = client.get(&url).send().await?;
  let project = resp.json::<Project>().await?;
  Ok(project)
}
