use crate::app::{App, AppState};
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

pub fn draw<B: Backend>(f: &mut Frame<B>, result: &Vec<(f64, f64)>, app: &mut App) {
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
        if app.state == AppState::ManualGame {
            let mut style = Style::default();
            if app.game.get_status(ii + 1).unwrap() {
                // Box is shut
                style = style.fg(Color::White).bg(Color::Blue);
            }
            let selected = ii == app.selection && app.tasks.state.selected().is_none();
            if selected {
                // Box is SELECTED
                style = style.fg(Color::Red);
            }
            if app.staging.iter().any(|&x| x == ii) {
                // Box is STAGED
                if selected {
                    style = style.fg(Color::Rgb(128, 0, 128));
                } else {
                    style = style.fg(Color::Blue);
                }
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
    // draw_plots(chunks[1]);

    let span_rolls = Spans::from(Span::styled(
        format!("Rolls: {:?}", app.game.get_rolls()),
        Style::default(),
    ));
    let span_wins = Spans::from(Span::styled(
        format!("Wins: {:?}", app.stats.num_won),
        Style::default(),
    ));
    let span_total = Spans::from(Span::styled(
        format!("Total: {:?}", app.stats.num_total),
        Style::default(),
    ));
    let display = vec![span_rolls, span_wins, span_total];
    let paragraph = Paragraph::new(display)
        .block(Block::default().title("Stats").borders(Borders::ALL))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);
    draw_plots(f, chunks[1], app);
}

fn draw_plots<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    match app.plotidx {
        0 => draw_hist_nshut(f, area, app),
        1 => draw_hist_rawrolls(f, area, app),
        2 => draw_hist_nrolls(f, area, app),
        _ => draw_hist_wins(f, area, app),
    }
}

fn draw_hist_nrolls<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let mut result: Vec<(f64, f64)> = Vec::new();
    for (ii, &total) in app.stats.count_nrolls.iter().enumerate() {
        result.push((ii as f64, total as f64));
    }

    let block = Block::default().title("Plots").borders(Borders::ALL);
    f.render_widget(block, area);
    let x_labels = vec![
        Span::raw(format!("{}", 0)),
        Span::raw(format!("{}", 6)),
        Span::raw(format!("{}", 12)),
    ];
    let datasets = vec![Dataset::default()
        .name("Count")
        .marker(symbols::Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .data(&result)];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Number of Rolls Per Game (Press P to Switch)",
                    Style::default().fg(Color::White),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("Rolls")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 12.0])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("Count")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 10.0])
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("0"),
                    Span::styled("10", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    f.render_widget(chart, area);
}

fn draw_hist_rawrolls<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let mut result: Vec<(f64, f64)> = Vec::new();
    for (ii, &total) in app.stats.count_rawrolls.iter().enumerate() {
        result.push((ii as f64, total as f64));
    }

    let block = Block::default().title("Plots").borders(Borders::ALL);
    f.render_widget(block, area);
    let x_labels = vec![
        Span::raw(format!("{}", 0)),
        Span::raw(format!("{}", 6)),
        Span::raw(format!("{}", 12)),
    ];
    let datasets = vec![Dataset::default()
        .name("Count")
        .marker(symbols::Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .data(&result)];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Dice Roll Count (Press P to Switch)",
                    Style::default().fg(Color::White),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("Rolls")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 12.0])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("Count")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 10.0])
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("0"),
                    Span::styled("10", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    f.render_widget(chart, area);
}

fn draw_hist_nshut<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let mut result: Vec<(f64, f64)> = Vec::new();
    for (ii, &total) in app.stats.count_shut.iter().enumerate() {
        result.push((ii as f64, total as f64));
    }

    let block = Block::default().title("Plots").borders(Borders::ALL);
    f.render_widget(block, area);
    let x_labels = vec![
        Span::raw(format!("{}", 0)),
        Span::raw(format!("{}", 6)),
        Span::raw(format!("{}", 12)),
    ];
    let datasets = vec![Dataset::default()
        .name("Count")
        .marker(symbols::Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .data(&result)];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Boxes Shut (Press P to Switch)",
                    Style::default().fg(Color::White),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("Box")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 12.0])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("Count")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 10.0])
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("0"),
                    Span::styled("10", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    f.render_widget(chart, area);
}

fn draw_hist_wins<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let mut sum = 0;
    let mut result: Vec<(f64, f64)> = Vec::new();
    for bounds in (0..1950).step_by(50).zip((50..2000).step_by(50)) {
        let count = app
            .stats
            .games_between_win
            .count_between(bounds.0, bounds.1 - 1);
        sum += count;
        result.push(((bounds.0 as f64 + bounds.1 as f64) / 2.0, count as f64));
    }

    let block = Block::default().title("Plots").borders(Borders::ALL);
    f.render_widget(block, area);
    let x_labels = vec![
        Span::raw(format!("{}", 0)),
        Span::raw(format!("{}", 1000)),
        Span::raw(format!("{}", 2000)),
    ];
    let datasets = vec![Dataset::default()
        .name("Count")
        .marker(symbols::Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .data(&result)];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Games Between Wins (Press P to Switch)",
                    Style::default().fg(Color::White),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("Games")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 2000.0])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("Count")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 200.0])
                .labels(vec![
                    Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("0"),
                    Span::styled("1000", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    f.render_widget(chart, area);
}
