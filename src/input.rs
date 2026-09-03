use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::ui::App;
use std::io;

pub fn start(app: &mut App) -> io::Result<()> {
    enable_raw_mode();

    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => { app.exit = true;  break; },
                    KeyCode::Char('t') => { println!("Yolo"); },
                    _ => { }
                }
            }
        }

    }

    disable_raw_mode();
    return Ok(());
}




































//pub fn start() -> io::Result<()> {
//    // Enable raw mode to read key presses instantly
//    enable_raw_mode()?;
//
//    println!("Press 't' (or 'q' to quit):");
//
//    loop {
//        // Read the next terminal event
//        if let Event::Key(key) = event::read()? {
//            // Check if it's a key press event (avoids double-firing on some platforms)
//            if key.kind == KeyEventKind::Press {
//                match key.code {
//                    KeyCode::Char('t') => {
//                        println!("\r\nYou pressed 't'!");
//                    }
//                    KeyCode::Char('q') => {
//                        break; // Exit loop on 'q'
//                    }
//                    _ => {}
//                }
//            }
//        }
//    }
//
//    // Always disable raw mode before exiting
//    disable_raw_mode()?;
//    Ok(())
//}

