use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::*,
};

use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut selected_tabs: usize = 0;
    loop {
        terminal.draw(|frame| {
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(5)])
                .split(frame.size());
            let tabs = Tabs::new(vec!["Test1", "Test2"])
                .block(Block::default().title("Tabs").borders(Borders::ALL))
                .select(selected_tabs);

            frame.render_widget(tabs, main_layout[0]);
        })?;

        // event
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Left | KeyCode::BackTab => {
                            if selected_tabs > 0 {
                                selected_tabs -= 1;
                            } 
                        }
                        KeyCode::Right | KeyCode::Tab => {
                            if selected_tabs < 1 {
                                selected_tabs += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
