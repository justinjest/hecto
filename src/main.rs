#![warn(clippy::all, clippy::pedantic, clippy::print_stdout, clippy::arithmetic_side_effects, clippy::as_conversions, clippy::integer_division)]

use std::io::Error;

mod editor;
use editor::Editor;

fn main() -> Result<(), Error> {
    let mut editor = Editor::default();
    editor.run()?;
    Ok(())
}
