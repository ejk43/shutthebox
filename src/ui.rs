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
    draw_plots(f, chunks[1], result);
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
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        if app.state == AppState::ManualGame {
            if app.staging.iter().any(|&x| x == ii) {
                paragraph = paragraph.style(Style::default().fg(Color::Green).bg(Color::Black));
            }
            if ii == app.selection {
                paragraph = paragraph.style(Style::default().fg(Color::Red).bg(Color::Black));
            }
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

fn draw_plots<B: Backend>(f: &mut Frame<B>, area: Rect, result: &Vec<(f64, f64)>) {
    let block = Block::default().title("Block 2").borders(Borders::ALL);
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
        .data(result)];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled("Chart", Style::default().fg(Color::Cyan)))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 2000.0])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
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
