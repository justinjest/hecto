use std::io::Error;
use crate::editor::term::{Term, Size};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
mod buffer;
use buffer::Buffer;

use crate::editor::Position;
#[derive(Default)]
pub struct View {
    pub buffer: Buffer,
}

impl View {

    pub fn render(&self) -> Result<(), Error> {
        if self.buffer.is_empty() == true {
            Self::draw_welcome_rows(&self)?;
        } else {
            Self::draw_buffer_rows(&self)?;
        }
        Ok(())
    }

    fn draw_welcome_rows(&self) -> Result<(), Error> {
        let Size{height, ..} = Term::size()?;
        for current_row in 0..height {
            // We don't need to put this exactly in the middle, it can be a
            // bit to the left or right
            #[allow(clippy::integer_division)]
            if current_row == height/3 {
                self.draw_row(current_row, &Self::create_welcome_message()?)?;
            } else {
                self.draw_row(current_row, "~")?;
            }
        }
        Ok(())
    }


    fn draw_buffer_rows(&self) -> Result<(), Error> {
        let Size{height, ..} = Term::size()?;
        for current_row in 0..height {
            // We don't need to put this exactly in the middle, it can be a
            // bit to the left or right
            #[allow(clippy::integer_division)]
            if let Some(element) = self.buffer.buf.get(current_row) {
                let elm = element;
                let msg = format!("{elm}");
                self.draw_row(current_row, &msg)?;
            }  else {
                self.draw_row(current_row, "~")?;
            }
        }
        Ok(())
    }


    fn create_welcome_message() -> Result<String, Error> {
        let mut msg = format!("{NAME} editor -- version {VERSION}");
        let width = Term::size()?.width;
        let len = msg.len();
        // We don't need to put this exactly in the middle, it can be a
        // bit to the left or right
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        msg = format!("~{spaces}{msg}");
        msg.truncate(width);
        Ok(msg)
        }

    fn draw_row(&self, line: usize, msg: &str) -> Result<(), Error> {
        Term::move_cursor_to(Position {x: 0, y: line})?;
        Term::print(msg)?;
        Ok(())
    }
}
