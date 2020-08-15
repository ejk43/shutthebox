use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
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
    let block = Block::default()
        .title("Shut the Box!")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    draw_boxes(f, chunks[0]);

    let chunks2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(0)].as_ref())
        .split(chunks[1]);
    draw_text(f, chunks2[0], app);
    draw_plots(f, chunks2[1], result);
}

fn draw_boxes<B: Backend>(f: &mut Frame<B>, area: Rect) {
    // let vert = Layout::default()
    //     .constraints([Constraint::Min(1), Constraint::Min(5), Constraint::Max(1)].as_ref())
    //     .split(area);
    let nboxes = 12;
    let constraints = vec![Constraint::Ratio(1, nboxes); nboxes as usize];
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(constraints)
        .split(area);
    for (ii, chunk) in chunks.iter().enumerate() {
        let block = Block::default()
            .title(format!("{}", ii + 1))
            .borders(Borders::ALL);
        f.render_widget(block, *chunk);
    }
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
    let tasks = List::new(tasks)
        .block(Block::default().borders(Borders::ALL).title("List"))
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
