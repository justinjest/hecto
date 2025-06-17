use crossterm::event::{read, Event, Event::Key, KeyCode, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::{Write, stdout, Error};
use std::cmp::{max, min};

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
        let mut caret_pos = Position{x: 0, y: 0};
        loop {
            self.refresh_screen(&caret_pos)?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            caret_pos = self.evaluate_event(&event, caret_pos)?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event, mut pos: Position) -> Result<Position, Error> {
        let Size{height, width} = Term::size()?;
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                KeyCode::Right => {pos = Position{x: min(pos.x.saturating_add(1), width-1), y: pos.y}},
                KeyCode::Left => {pos = Position{x: max(pos.x.saturating_sub(1), 0), y: pos.y}},
                KeyCode::Up => {pos = Position{x: pos.x, y: min(pos.y.saturating_sub(1), height-2)}},
                KeyCode::Down => {pos = Position{x: pos.x, y: max(pos.y.saturating_add(1), 0)}},
                KeyCode::PageUp => {pos = Position{x: pos.x, y: 0}},
                KeyCode::PageDown => {pos = Position{x: pos.x, y: height-1}},
                KeyCode::Home => {pos = Position{x: 0, y: pos.y}},
                KeyCode::End => {pos = Position{x: width-1, y: pos.y}},
                _ => (),
            }
        }
        Ok(pos)
    }
    fn refresh_screen(&self, pos:&Position) -> Result<(), Error> {
        Term::hide_cursor()?;
        Term::move_cursor_to(Position{x:0, y:0})?;
        stdout().flush()?;
        if self.should_quit {
            Term::update_screen()?;
            Term::print("Goodbye!\r\n")?;
        } else {
            Self::draw_rows()?;
            Term::move_cursor_to(*pos)?;
        }
        Term::show_cursor()?;
        Term::execute()?;
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
