use crossterm::{terminal::{disable_raw_mode, enable_raw_mode}};
use ratatui::{self, DefaultTerminal, Frame, layout::{Constraint, Layout, Rect}, prelude::Color, style::{Style, Stylize}, symbols::border, text::Line, widgets::{Block, Gauge, Widget}};
use std::io;
use crate::input;

pub struct App {
    pub exit: bool,
    pub progress_bar_color: Color,
    pub progress_bar_color_index: usize,
}

impl App {
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
        let vertical_layout = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let [title_area, gauge_area] = vertical_layout.areas(area);
        Line::from("Proccess overview").bold().render(area, buf);

        let instructions = Line::from(vec![
            " Change color ".into(),
            "<C>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]).centered();

        let block = Block::bordered()
            .title(Line::from(" Background processes "))
            .title_bottom(instructions)
            .border_set(border::THICK);

        let progress_bar = Gauge::default()
            .gauge_style(Style::default().fg(self.progress_bar_color))
            .block(block)
            .label(format!("Process 1: 50%"))
            .ratio(0.5);

        progress_bar.render(Rect {
            x: gauge_area.left(),
            y: gauge_area.top(),
            width: gauge_area.width,
            height: 3,
        }, buf);
    }
}

pub fn init_app() -> App {
    let mut terminal = ratatui::init();
    let mut app = App {
        exit: false,
        progress_bar_color: Color::White,
        progress_bar_color_index: 0,
    };

    let _ = app.run(&mut terminal);
    ratatui::restore();
    
    return app;
}

