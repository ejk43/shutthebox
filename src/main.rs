#[allow(dead_code)]
mod app;
mod event;
pub mod game;
mod ui;

use crate::app::App;
use crate::event::{Config, Event, Events};

use std::time::{Duration, Instant};
use std::{error::Error, io};
use termion::clear;
use termion::raw::IntoRawMode;
use termion::{event::Key, input::MouseTerminal, screen::AlternateScreen};
use tui::backend::TermionBackend;
use tui::Terminal;

use rayon::prelude::*;
use std::sync::Mutex;

fn main() -> Result<(), Box<dyn Error>> {
    // println!("{}", clear::All);
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(250),
        ..Config::default()
    });

    let mut app = App::new("Shut the Box!");
    app.tasks.state.select(Some(0));

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char(c) => app.on_key(c),
                Key::Up => app.on_up(),
                Key::Down => app.on_down(),
                Key::Left => app.on_left(),
                Key::Right => app.on_right(),
                _ => {}
            },
            Event::Tick => app.on_tick(),
        }
        if app.should_quit {
            break;
        }
    }
    // println!("{}", clear::All);
    Ok(())
}
