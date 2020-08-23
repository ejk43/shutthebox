use crate::app::{App, AppState};
use std::cmp;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, List, ListItem, ListState,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(12), Constraint::Min(0)].as_ref())
        .split(f.size());

    draw_boxes(f, chunks[0], app);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(0)].as_ref())
        .split(chunks[1]);
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
            .split(chunks[0]);
        {
            draw_dice(f, chunks[0], app);
            draw_text(f, chunks[1], app);
        }
    }
    draw_stats(f, chunks[1], app);
}

fn draw_boxes<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let block = Block::default()
        .title("Shut the Box!")
        .borders(Borders::ALL);
    f.render_widget(block, area);

    let nboxes = 12;
    let constraints = vec![Constraint::Ratio(1, nboxes); nboxes as usize];
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(constraints)
        .split(area);
    for (ii, chunk) in chunks.iter().enumerate() {
        // let block = Block::default()
        //     .title(format!("{}", ii + 1))
        //     .borders(Borders::ALL);
        // f.render_widget(block, *chunk);
        let halfway = (chunk.height as f32 / 2.0).floor() as usize - 2;
        let mut text = vec![Spans::from(Span::raw("")); halfway];
        text.push(Spans::from(Span::raw(format!("{}", ii + 1))));
        let mut paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        if app.state != AppState::Idle {
            let mut style = Style::default();
            if app.game.get_status(ii + 1).unwrap() {
                // Box is shut
                style = style.fg(Color::White).bg(Color::Blue);
            }
            let selected = ii == app.selection && app.tasks.state.selected().is_none();
            let staged = app.staging.iter().any(|&x| x == ii);
            if selected && app.state == AppState::ManualGame {
                // Box is SELECTED
                style = if staged {
                    style.fg(Color::Rgb(128, 0, 128))
                } else {
                    style.fg(Color::Red)
                };
            } else if staged {
                style = style.fg(Color::Blue);
            }
            paragraph = paragraph.style(style)
        }
        f.render_widget(paragraph, *chunk);
    }
}

fn draw_dice<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let text = match app.state {
        AppState::ManualGame => vec![
            Spans::from(Span::styled(
                app.dice.pprint(),
                Style::default().fg(Color::Red),
            )),
            Spans::from(Span::styled(
                format!("ROLL = {}", app.dice.result()),
                Style::default().fg(Color::Red),
            )),
            Spans::from(Span::styled(
                format!(
                    "TOTAL = {}",
                    app.staging.iter().fold(0, |acc, x| acc + x + 1)
                ),
                Style::default().fg(Color::Red),
            )),
        ],
        _ => vec![
            Spans::from(Span::styled("⚀ ⚁ ⚂ ⚃ ⚄ ⚅", Style::default().fg(Color::Red))),
            Spans::from(Span::styled("ROLL", Style::default().fg(Color::Red))),
            Spans::from(Span::styled("TOTAL", Style::default().fg(Color::Red))),
        ],
    };
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Dice").borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_text<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    // let block2 = Block::default().title("Selection").borders(Borders::ALL);
    // f.render_widget(block2, area);

    // Draw tasks
    let tasks: Vec<ListItem> = app
        .tasks
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(*i))]))
        .collect();
    let title = match app.state {
        AppState::ManualGame => "Playing!",
        _ => "Select Game",
    };
    let tasks = List::new(tasks)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(tasks, area, &mut app.tasks.state);
}

fn draw_stats<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(40), Constraint::Min(0)].as_ref())
        .split(area);
    {
        let stats = app.stats.lock().unwrap();
        let span_rolls = Spans::from(Span::styled(
            format!("Rolls: {:?}", app.game.get_rolls()),
            Style::default(),
        ));
        let span_wins = Spans::from(Span::styled(
            format!("Wins: {:?}", stats.num_won),
            Style::default(),
        ));
        let span_total = Spans::from(Span::styled(
            format!("Total: {:?}", stats.num_total),
            Style::default(),
        ));
        let display = vec![span_rolls, span_wins, span_total];
        let paragraph = Paragraph::new(display)
            .block(Block::default().title("Stats").borders(Borders::ALL))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[0]);
    }
    draw_plots(f, chunks[1], app);
}

fn draw_plots<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    match app.plotidx {
        0 => draw_hist_nshut(f, area, app),
        1 => draw_hist_rawrolls(f, area, app),
        2 => draw_hist_nrolls(f, area, app),
        3 => draw_hist_lastroll(f, area, app),
        _ => draw_hist_wins(f, area, app),
    }
}

fn create_chart<'a>(
    data: &'a Vec<(f64, f64)>,
    title: &'a str,
    xaxis: &'a str,
    yaxis: &'a str,
) -> Chart<'a> {
    let xmax = data.iter().map(|&x| x.0 as u64).max().unwrap() as f64;
    let xhalf = (xmax + 1.0) / 2.0;
    let ymax = data.iter().map(|&x| x.1 as u64).max().unwrap() as f64;
    let nextpow10 = cmp::max(10, 10_u64.pow(ymax.log10().ceil() as u32)) as f64;
    let x_labels = vec![
        Span::raw(format!("{}", 1)),
        Span::raw(format!("{}", xhalf as u64)),
        Span::raw(format!("{}", xmax as u64 + 1)),
    ];
    let y_labels = vec![
        Span::styled(
            format!("{}", 1),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{}", ((nextpow10 as f64) / 2.0) as u64),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{}", nextpow10 as u64),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ];
    let datasets = vec![Dataset::default()
        .name("Count")
        .marker(symbols::Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .data(data)];
    Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(title, Style::default().fg(Color::White)))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title(xaxis)
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, xmax])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title(yaxis)
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, nextpow10])
                .labels(y_labels),
        )
}

fn draw_hist_nrolls<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let mut result: Vec<(f64, f64)> = Vec::new();
    {
        let stats = app.stats.lock().unwrap();
        for (ii, &total) in stats.count_nrolls.iter().enumerate() {
            result.push((ii as f64, total as f64));
        }
    }

    let block = Block::default().title("Plots").borders(Borders::ALL);
    f.render_widget(block, area);
    let chart = create_chart(
        &result,
        "Number of Rolls Per Game (Press P to Switch)",
        "Rolls",
        "Count",
    );
    f.render_widget(chart, area);
}

fn draw_hist_rawrolls<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let mut result: Vec<(f64, f64)> = Vec::new();
    {
        let stats = app.stats.lock().unwrap();
        for (ii, &total) in stats.count_rawrolls.iter().enumerate() {
            result.push((ii as f64, total as f64));
        }
    }
    let block = Block::default().title("Plots").borders(Borders::ALL);
    f.render_widget(block, area);
    let chart = create_chart(
        &result,
        "Dice Roll Count (Press P to Switch)",
        "Rolls",
        "Count",
    );
    f.render_widget(chart, area);
}

fn draw_hist_nshut<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let mut result: Vec<(f64, f64)> = Vec::new();
    {
        let stats = app.stats.lock().unwrap();
        for (ii, &total) in stats.count_shut.iter().enumerate() {
            result.push((ii as f64, total as f64));
        }
    }

    let block = Block::default().title("Plots").borders(Borders::ALL);
    f.render_widget(block, area);
    let chart = create_chart(&result, "Boxes Shut (Press P to Switch)", "Box", "Count");
    f.render_widget(chart, area);
}

fn draw_hist_lastroll<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let mut result: Vec<(f64, f64)> = Vec::new();
    {
        let stats = app.stats.lock().unwrap();
        for (ii, &total) in stats.count_lastroll.iter().enumerate() {
            result.push((ii as f64, total as f64));
        }
    }

    let block = Block::default().title("Plots").borders(Borders::ALL);
    f.render_widget(block, area);
    let chart = create_chart(&result, "Losing Roll (Press P to Switch)", "Rolls", "Count");
    f.render_widget(chart, area);
}

fn arange(min: u64, max: u64, step: u64) -> impl Iterator<Item = (u64, u64)> {
    (min..max)
        .step_by(step as usize)
        .zip(((min + step)..(max + step)).step_by(step as usize))
}

fn draw_hist_wins<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let mut sum = 0;
    let mut result: Vec<(f64, f64)> = Vec::new();
    {
        let stats = app.stats.lock().unwrap();
        for bounds in arange(0, 2000, 25) {
            let count = stats
                .games_between_win
                .count_between(bounds.0, bounds.1 - 1);
            sum += count;
            result.push((bounds.1 as f64, count as f64));
        }
    }

    let block = Block::default().title("Plots").borders(Borders::ALL);
    f.render_widget(block, area);
    let chart = create_chart(
        &result,
        "Games Between Wins (Press P to Switch)",
        "Games",
        "Count",
    );
    f.render_widget(chart, area);
}
