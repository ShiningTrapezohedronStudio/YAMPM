use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CoreConfig {
  pub project_name: String,
}
