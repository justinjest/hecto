use std::io::Error;

use crate::editor::term::{Term, Size};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View;
impl View {
    pub fn render() -> Result<(), Error> {
        Self::draw_rows()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size{height, ..} = Term::size()?;
        for current_row in 0..height {
            Term::clear_line()?;
            // We don't need to put this exactly in the middle, it can be a
            // bit to the left or right
            #[allow(clippy::integer_division)]
            if current_row == height/3 {
            Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row.saturating_add(1) < height {
                Term::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
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
        Term::print(msg)?;
        Ok(())
        }

    fn draw_empty_row() -> Result<(), Error> {
        Term::print("~")?;
        Ok(())
    }

}
