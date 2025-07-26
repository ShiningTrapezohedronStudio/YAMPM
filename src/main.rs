mod curseforge;
mod cli;
mod config;

use rustyline::DefaultEditor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  DefaultEditor::new()?
    .readline("YAMPM: ")
    .map_err(|e| e.to_string())
    .and_then(|input| {
      if input.trim().is_empty() {
        Err("Input cannot be empty".into())
      } else {
        Ok(())
      }
    })?;
    Ok(())
}
