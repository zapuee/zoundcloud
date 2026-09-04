use crossterm::{terminal::{disable_raw_mode, enable_raw_mode}};
use ratatui::{
    self,
    DefaultTerminal,
    Frame,
    layout::{Constraint, Layout, Rect},
    symbols::border,
    widgets::{Block, List, ListItem, Widget}
};
use std::{io, str};
use crate::input;

pub struct App {
    pub exit: bool,
    pub songs: Vec<String>
}

impl App {
    fn new() -> Self {
        Self {
            exit: false,
            songs: Vec::new(),
        }
    }

    fn add_song(&mut self, song: &str) -> io::Result<()> {
        self.songs.push(song.to_string());
        return Ok(());
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let _ = enable_raw_mode();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            input::handle_frame(self)?;
        }
        let _ = disable_raw_mode();
        return Ok(());
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(
        self,
        area: ratatui::prelude::Rect, 
        buf: &mut ratatui::prelude::Buffer
    )
    where
        Self: Sized,
    {
        let vertical_layout = Layout::vertical([
            Constraint::Percentage(20),
            Constraint::Percentage(80)
        ]);

        let [title_area, gauge_area] = vertical_layout.areas(area);

        let window_border = Block::bordered()
            .border_set(border::PLAIN);
        window_border.render(area, buf);

        let song_list = List::new(self.songs.clone())
            .block(Block::bordered());
        song_list.render(gauge_area, buf);
    }
}

pub fn init_app() -> App {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    for _ in 0..10 {
        app.add_song("apples");
    }

    let _ = app.run(&mut terminal);
    ratatui::restore();
    
    return app;
}

