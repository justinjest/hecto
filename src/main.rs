#![warn(clippy::all, clippy::pedantic, clippy::print_stdout, clippy::arithmetic_side_effects, clippy::as_conversions, clippy::integer_division)]

use std::fs;
use std::io::Error;

mod editor;
use editor::Editor;

fn main() -> Result<(), Error> {
    let mut editor = Editor::default();
    let args: Vec<String> = std::env::args().collect();
    if let Some(first_arg) = args.get(1) {
        let file_contents = std::fs::read_to_string(first_arg)?;
        editor.load(file_contents);
        // Some function that we have an arg
    }
    editor.run();
    Ok(())
}
