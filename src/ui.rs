use ratatui::{self, DefaultTerminal, Frame, style::Stylize, text::Line};
use std::io;

pub struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
        }
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
        Line::from("Proccess overview").bold().render(area, buf);
    }
}

pub fn init_terminal() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App {
        exit: false
    };

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    
    return app_result;
}

pub fn run_ui() -> io::Result<()> {
    

    return Ok(());
}
