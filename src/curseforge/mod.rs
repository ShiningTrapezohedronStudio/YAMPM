use std::path::Path;

#[cfg(not(windows))]
pub(crate) fn curse_dir() -> Option<Path> { None }

// #[cfg(windows)]
pub(crate) fn curse_dir() -> Option<&Path> {
  let home_dir = directories::BaseDirs::new()?.home_dir();

  let curse_app_install_dir = home_dir.join("Curse");
  let twitch_app_install_dir = home_dir.join("Twitch");

  if (curse_app_install_dir.try_exists().or(false)) {
    return Ok(curse_app_install_dir.as_path());
  }

  if (twitch_app_install_dir.try_exists().or(false)) {
    return Ok(twitch_app_install_dir.as_path());
  }

  None
}
