use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::style::Color;
use crate::app::App;
use std::io;

const COLOR_ORDER: [Color; 5] = [
    Color::Yellow,
    Color::Green,
    Color::Blue,
    Color::Red,
    Color::White
];

pub fn handle_frame(app: &mut App) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q') => { app.exit = true; },
            KeyCode::Char('t') => { println!("yolo"); },
            KeyCode::Char('c') => { 
                app.progress_bar_color_index = (app.progress_bar_color_index + 1) % COLOR_ORDER.len();
                app.progress_bar_color = *COLOR_ORDER.get(app.progress_bar_color_index).unwrap();
            },
            _ => { }
        }
    }
    return Ok(());
}

