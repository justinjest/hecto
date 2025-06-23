use crossterm::event::{read, Event, Event::Key, KeyCode, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;
use std::cmp::{max, min};

mod term;
use term::{Term, Size, Position};
mod view;
use view::View;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {

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
               }
                KeyCode::Right => {pos = Position{x: min(pos.x.saturating_add(1), width-1), y: pos.y}}
                KeyCode::Left => {pos = Position{x: max(pos.x.saturating_sub(1), 0), y: pos.y}}
                KeyCode::Up => {pos = Position{x: pos.x, y: min(pos.y.saturating_sub(1), height-2)}}
                KeyCode::Down => {pos = Position{x: pos.x, y: max(pos.y.saturating_add(1), 0)}}
                KeyCode::Home => {pos = Position{x: 0, y: pos.y}}
                KeyCode::End => {pos = Position{x: width, y: pos.y}}
                KeyCode::PageUp => {pos = Position{x: pos.x, y: 0}}
                KeyCode::PageDown => {pos = Position{x: pos.x, y: height}}
                _ => (),
            }
        }
        Ok(pos)
    }

    // (Max (Min current pos || screen size) 0)
    fn refresh_screen(&mut self, pos:&Position) -> Result<(), Error> {
        Term::hide_cursor()?;
        Term::move_cursor_to(Position{x:0, y:0})?;
        if self.should_quit {
            Term::update_screen()?;
            Term::print("Goodbye!\r\n")?;
        } else {
            View::render(&self.view)?;
            Term::move_cursor_to(*pos)?;
        }
        Term::show_cursor()?;
        Term::execute()?;
        Ok(())
    }

}
