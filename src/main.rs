pub mod game;

use std::time::{Duration, Instant};

use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
// use tui::layout::{Constraint, Direction, Layout};
// use tui::widgets::{Block, Borders, Widget};
use termion::{event::Key, input::MouseTerminal, screen::AlternateScreen};
use tui::Terminal;

use termion::clear;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, List, ListItem, Paragraph, Row,
        Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

// fn main() -> Result<(), io::Error> {
//     println!("{}", clear::All);
//     let result = accumulate_stats(1_000_000);
//     let stdout = io::stdout().into_raw_mode()?;
//     // let stdout = MouseTerminal::from(stdout);
//     // let stdout = AlternateScreen::from(stdout);
//     let backend = TermionBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;
//     terminal.draw(|mut f| {
//         let chunks = Layout::default()
//             .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
//             .split(f.size());
//         let block = Block::default().title("Block").borders(Borders::ALL);
//         f.render_widget(block, chunks[0]);
//         let block = Block::default().title("Block 2").borders(Borders::ALL);
//         f.render_widget(block, chunks[1]);

//         let x_labels = vec![
//             Span::raw(format!("{}", 0)),
//             Span::raw(format!("{}", 1000)),
//             Span::raw(format!("{}", 2000)),
//         ];
//         let datasets = vec![Dataset::default()
//             .name("Count")
//             .marker(symbols::Marker::Dot)
//             .style(Style::default().fg(Color::Cyan))
//             .data(&result)];
//         let chart = Chart::new(datasets)
//             .block(
//                 Block::default()
//                     .title(Span::styled("Chart", Style::default().fg(Color::Cyan)))
//                     .borders(Borders::ALL),
//             )
//             .x_axis(
//                 Axis::default()
//                     .title("X Axis")
//                     .style(Style::default().fg(Color::Gray))
//                     .bounds([0.0, 2000.0])
//                     .labels(x_labels),
//             )
//             .y_axis(
//                 Axis::default()
//                     .title("Y Axis")
//                     .style(Style::default().fg(Color::Gray))
//                     .bounds([0.0, 1000.0])
//                     .labels(vec![
//                         Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
//                         Span::raw("0"),
//                         Span::styled("1000", Style::default().add_modifier(Modifier::BOLD)),
//                     ]),
//             );
//         f.render_widget(chart, chunks[1]);
//     })
// }

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

use rayon::prelude::*;
use std::sync::mpsc::channel;
use std::sync::Mutex;

fn accumulate_stats_par(total: i64) -> Vec<(f64, f64)> {
    let start = Instant::now();
    let mut statsmutex = Mutex::new(game::Statistics::new());

    (0..total).into_par_iter().for_each(|_| {
        let game = game::simulate_game();
        let mut stats = statsmutex.lock().unwrap();
        stats.save_game(&game);
    });

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    let mut stats = statsmutex.lock().unwrap();
    let mut sum = 0;
    let mut hist = Vec::new();
    for bounds in (0..1950).step_by(50).zip((50..2000).step_by(50)) {
        let count = stats
            .games_between_win
            .count_between(bounds.0, bounds.1 - 1);
        println!(
            "Amount between {} and {}: {}",
            bounds.0,
            bounds.1 - 1,
            count
        );
        sum += count;
        hist.push(((bounds.0 as f64 + bounds.1 as f64) / 2.0, count as f64));
    }
    println!("total: {}", sum);
    hist
}

fn accumulate_stats(total: i64) -> Vec<(f64, f64)> {
    let start = Instant::now();
    let mut stats = game::Statistics::new();
    for _ in 0..total {
        let game = game::simulate_game();
        let rolls = game.get_rolls();
        if !game.check_loss(rolls[rolls.len() - 1]) {
            println!("Funky game! {:?}", game)
        }
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

fn main() {
    // play_until_victory();
    // let nruns = 1_000_000;
    // accumulate_stats(1000);
    // accumulate_stats_par(nruns);

    let mut game = game::ShutTheBox::init(12);
    // assert_eq!(game.check_loss(10), false);
    assert_eq!(game.check_loss(10), false);
    game.shut(10);
    assert_eq!(game.check_loss(10), false);
    game.shut(9);
    assert_eq!(game.check_loss(10), false);
    game.shut(8);
    assert_eq!(game.check_loss(10), false);
    game.shut(7);
    assert_eq!(game.check_loss(10), false);
    game.shut(6);
    assert_eq!(game.check_loss(10), false);
    game.shut(4);
    assert_eq!(game.check_loss(10), false);
    game.shut(1);
    assert_eq!(game.check_loss(10), false);
    game.shut(2);
    assert_eq!(game.check_loss(10), true);
}
