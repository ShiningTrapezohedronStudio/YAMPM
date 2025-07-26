mod cli;
mod config;
mod launchers;
mod sources;
mod repl;

// use crate::repl::repl;
use crate::sources::modrinth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let project = modrinth::fetch_project("create").await?;
  println!("{:#?}", project);

  // repl()
  //   .map_err(|e| {
  //     eprintln!("Error: {}", e);
  //     e
  //   })
  //   .and_then(|_| {
  //     println!("Exiting REPL.");
  //     Ok(())
  //   })

  Ok(())
}
