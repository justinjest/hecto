use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::{Write, stdout, Error};


mod term;
use term::{Term, Size, Position};

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
            Term::print("~")?;
            if current_row + 1 < height {
                Term::print("\r\n")?;
            }
            stdout().flush()?;
        }
        Ok(())
    }

}
