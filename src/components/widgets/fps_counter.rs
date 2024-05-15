use anyhow::Result;
use ratatui::{prelude::*, widgets::*};
use std::time::Instant;

pub struct FpsCounter {
    app_start_time: Instant,
    app_frames: u32,
    app_fps: f64,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl FpsCounter {
    pub fn new() -> Self {
        FpsCounter {
            app_start_time: Instant::now(),
            app_frames: 0,
            app_fps: 0.0,
        }
    }

    pub fn app_tick(&mut self) {
        self.app_frames += 1;
        let now = Instant::now();
        let elapsed = (now - self.app_start_time).as_secs_f64();
        if elapsed >= 1.0 {
            self.app_fps = self.app_frames as f64 / elapsed;
            self.app_start_time = now;
            self.app_frames = 0;
        }
    }

    pub fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1), // first row
                Constraint::Min(0),
            ])
            .split(rect);

        let rect = rects[0];

        let s = format!("{:.2} ticks per sec (app) ", self.app_fps);
        let text = Span::from(s);
        f.render_widget(text, rect);
        Ok(())
    }
}
