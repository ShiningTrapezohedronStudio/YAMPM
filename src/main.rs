mod cli;
mod config;
mod launchers;
mod repl;

use crate::repl::repl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  repl()
    .map_err(|e| {
      eprintln!("Error: {}", e);
      e
    })
    .and_then(|_| {
      println!("Exiting REPL.");
      Ok(())
    })
}
