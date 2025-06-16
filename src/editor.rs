use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::{Write, stdout, Error};


mod term;
use term::{Term, Size, Position};


const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }
    pub fn run(&mut self) {
        Term::initialize().unwrap();
        let result = self.repl();
        Term::terminate().unwrap();
        result.unwrap();
    }
    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        Term::hide_cursor()?;
        stdout().flush()?;
        if self.should_quit {
            Term::update_screen()?;
            Term::print("Goodbye!\r\n")?;
        } else {
            Self::draw_rows()?;
            Term::move_cursor_to(Position{x:0,y:0})?;
        }
        Term::show_cursor()?;
        Term::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size{height, ..} = Term::size()?;
        for current_row in 0..height {
            Term::clear_line()?;
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
