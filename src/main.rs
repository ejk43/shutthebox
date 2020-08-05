#[allow(dead_code)]
mod event;
pub mod game;
mod ui;

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
    let result = accumulate_stats_par(100_000);
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(250),
        ..Config::default()
    });

    loop {
        terminal.draw(|f| ui::draw(f, &result))?;

        let mut should_quit = false;
        match events.next()? {
            Event::Input(key) => match key {
                Key::Char(c) => match c {
                    'q' => {
                        should_quit = true;
                    }
                    _ => {}
                },
                _ => {}
            },
            Event::Tick => {}
        }
        if should_quit {
            break;
        }
    }
    // println!("{}", clear::All);
    Ok(())
}

fn play_until_victory() {
    let mut won = false;
    let mut niter = 0;
    while !won {
        let game = game::simulate_game();
        won = game.victory();
        niter += 1;
        println!(
            "{} Try number {}",
            match won {
                true => "WON!!",
                false => "Lost :(",
            },
            niter
        );
        println!("Rolls: {:?}", game.get_rolls());
        println!("Shut: {:?}", game.get_numbers());
    }
}

fn accumulate_stats_par(total: i64) -> Vec<(f64, f64)> {
    let start = Instant::now();
    let mut statsmutex = Mutex::new(game::Statistics::new());

    (0..total).into_par_iter().for_each(|_| {
        let game = game::simulate_game();
        let mut stats = statsmutex.lock().unwrap();
        stats.save_game(&game);
    });

    let duration = start.elapsed();
    // println!("Time elapsed in expensive_function() is: {:?}", duration);

    let stats = statsmutex.lock().unwrap();
    let mut sum = 0;
    let mut hist = Vec::new();
    for bounds in (0..1950).step_by(50).zip((50..2000).step_by(50)) {
        let count = stats
            .games_between_win
            .count_between(bounds.0, bounds.1 - 1);
        // println!(
        //     "Amount between {} and {}: {}",
        //     bounds.0,
        //     bounds.1 - 1,
        //     count
        // );
        sum += count;
        hist.push(((bounds.0 as f64 + bounds.1 as f64) / 2.0, count as f64));
    }
    // println!("total: {}", sum);
    hist
}

fn accumulate_stats(total: i64) -> Vec<(f64, f64)> {
    let start = Instant::now();
    let mut stats = game::Statistics::new();
    for _ in 0..total {
        let game = game::simulate_game();
        let rolls = game.get_rolls();
        // if !game.check_loss(rolls[rolls.len() - 1]) {
        //     println!("Funky game! {:?}", game)
        // }
        stats.save_game(&game);
        // println!("One Game: {:?}", game);
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    // println!("Stats: {:?}", stats);
    for pct in (1..100).step_by(5) {
        // println!(
        //     "{}'th percentile of data is {}",
        //     pct,
        //     stats
        //         .games_between_win
        //         .value_at_quantile((pct as f64) / 100.0)
        // );
    }
    let mut sum = 0;
    let mut hist = Vec::new();
    for bounds in (0..1950).step_by(50).zip((50..2000).step_by(50)) {
        let count = stats
            .games_between_win
            .count_between(bounds.0, bounds.1 - 1);
        // println!(
        //     "Amount between {} and {}: {}",
        //     bounds.0,
        //     bounds.1 - 1,
        //     count
        // );
        sum += count;
        hist.push(((bounds.0 as f64 + bounds.1 as f64) / 2.0, count as f64));
    }
    // println!("total: {}", sum);
    hist
}

// fn main() {
//     // play_until_victory();
//     // let nruns = 10_000_000;
//     // let nruns = 1000;
//     // accumulate_stats(nruns);
//     // accumulate_stats_par(nruns);
// }
