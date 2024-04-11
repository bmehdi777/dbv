use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};

use dbv::{app::App, events::events::*, log::set_logger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_terminal()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new();
    let events_handling = EventsHandling::new().start();

    app.app_state
        .config
        .init()
        .expect("An error occured while initializing the config file.");

    set_logger(&app.app_state.config)?;

    loop {
        // draw
        terminal
            .draw(|frame| {
                if let Err(e) = app.draw(frame) {
                    eprintln!("An error occured : {}", e.to_string());
                    std::process::exit(1);
                };
            })
            .expect("An error occured while rendering terminal.");

        // event
        match events_handling.next() {
            Ok(event) => {
                if let EventThread::Event(key) = event {
                    app.event_handling(key).unwrap();
                }
            }
            Err(err) => {
                eprintln!("An error occured : {}", err.to_string());
                std::process::exit(1);
            }
        };

        if app.app_state.exit {
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
