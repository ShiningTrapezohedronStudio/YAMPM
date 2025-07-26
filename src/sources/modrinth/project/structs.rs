use serde::Deserialize;

use super::enums::*;

#[derive(Debug, Deserialize)]
pub struct License {
  id: String,
  name: String,
  url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GalleryImage {
  url: String,
  featured: bool,
  title: Option<String>,
  description: Option<String>,
  created: String,
  ordering: u32,
}

#[derive(Debug, Deserialize)]
pub struct ModeratorMessage {
  message: String,
  body: String,
}

#[derive(Debug, Deserialize)]
pub struct DonationUrl {
  id: String,
  platform: String,
  url: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
  id: String,
  slug: String,
  team: String,
  title: String,
  description: String,
  categories: Vec<String>,
  client_side: ClientSide,
  server_side: ServerSide,
  body: String,
  status: Status,
  requested_status: Option<RequestedStatus>,
  additional_categories: Vec<String>,
  issues_url: Option<String>,
  source_url: Option<String>,
  wiki_url: Option<String>,
  discord_url: Option<String>,
  donation_urls: Vec<DonationUrl>,
  project_type: ProjectType,
  downloads: u32,
  followers: u32,
  icon_url: Option<String>,
  color: Option<u32>,
  thread_id: String,
  monetization_status: MonetizationStatus,
  moderator_message: Option<ModeratorMessage>,
  published: String,
  updated: String,
  approved: Option<String>,
  queued: Option<String>,
  license: Option<License>,
  versions: Vec<String>,
  game_versions: Vec<String>,
  loaders: Vec<String>,
  gallery: Vec<GalleryImage>,
}
