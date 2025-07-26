use rustyline::DefaultEditor;

pub(super) fn repl() -> Result<(), Box<dyn std::error::Error>> {
  let mut editor = DefaultEditor::new()?;

  loop {
    editor
      .readline("YAMPM: ")
      .map_err(|e| e.to_string())
      .and_then(|input| {
        if input.trim().is_empty() {
          Err("Input cannot be empty".into())
        } else {
          Ok(())
        }
      })?;
  }
}
