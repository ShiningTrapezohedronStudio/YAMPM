use std::path::PathBuf;

#[cfg(not(windows))]
pub(crate) fn curse_dir() -> Option<PathBuf> { None }

#[cfg(windows)]
pub(crate) fn curse_dir() -> Option<PathBuf> {
  let install_locations = [
    "Curse",
    "Twitch",
  ];

  let base_dirs: directories::BaseDirs = directories::BaseDirs::new()?;
  let home_dir = base_dirs.home_dir();

  install_locations
    .iter()
    .map(|d| home_dir.join(d))
    .find(|p| p.try_exists().unwrap_or(false))
}
