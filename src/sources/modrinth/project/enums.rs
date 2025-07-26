use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClientSide {
  Required,
  Optional,
  Unsupported,
  Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerSide {
  Required,
  Optional,
  Unsupported,
  Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
  Approved,
  Archived,
  Rejected,
  Draft,
  Unlisted,
  Processing,
  Withheld,
  Scheduled,
  Private,
  Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestedStatus {
  Approved,
  Archived,
  Unlisted,
  Private,
  Draft,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
  Mod,
  Modpack,
  ResourcePack,
  Shader,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MonetizationStatus {
  Monetized,
  Demonetized,
  ForceDemonetized,
}
