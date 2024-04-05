use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};

use dbv::app::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_terminal()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    terminal.clear()?;

    let mut app = App::new();

    loop {
        // draw
        terminal.draw(|frame| {
            if let Err(e) = app.draw(frame) {
                eprintln!("An error occured : {}", e.to_string());
                std::process::exit(1);
            };
        });

        // event
        app.event_handling();
        if app.exit {
            break;
        }
    }

    close_terminal()?;
    Ok(())
}

fn setup_terminal() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    Ok(())
}

fn close_terminal() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
