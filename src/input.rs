use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::app::App;
use std::io;

pub fn handle_frame(app: &mut App) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q') => { app.exit = true; },
            KeyCode::Char('t') => { println!("yolo"); },
            _ => { }
        }
    }
    return Ok(());
}

