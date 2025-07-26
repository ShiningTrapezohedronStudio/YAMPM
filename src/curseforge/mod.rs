use std::path::Path;

#[cfg(not(windows))]
pub(crate) fn curse_dir() -> Option<Path> { None }

#[cfg(windows)]
pub(crate) fn curse_dir() -> Option<&Path> {
  let install_locations = [
    "Curse",
    "Twitch",
  ];

  let home_dir = directories::BaseDirs::new()?.home_dir();

  install_locations
    .iter()
    .map(|d| home_dir.join(d))
    .find(|p| p.try_exists().unwrap_or(false))
    .map(|p| p.as_path())
}
