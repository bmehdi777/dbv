use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};

use dbv::{application::App, events::events::*, log::set_logger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_terminal()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let events_handling = EventsHandling::new().start();
    let mut app = App::new(&events_handling);

    app.store
        .preference
        .init()
        .expect("An error occured while initializing the config file.");

    sqlx::any::install_default_drivers();

    set_logger(&app.store.preference)?;

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

        // update
        app.store.update();

        if app.store.exit {
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
