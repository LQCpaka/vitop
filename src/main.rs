use std::io::{self};

use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::app::app::App;

pub mod app;
pub mod collector;
pub mod error;
pub mod types;
pub mod ui;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    //open a sub terminal
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // Start app state
    let mut app = App::new();

    // main loop
    while !app.should_quit {
        terminal.draw(|_f| {})?;

        // event listener - key input
        // wait for 50ms for input
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // press 'q' to set should_quit (appstat) from 'false' to 'true'
                if key.code == KeyCode::Char('q') {
                    app.quit();
                }
            }
        }
    }

    // clean before quit
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
