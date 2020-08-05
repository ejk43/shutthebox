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

pub fn draw<B: Backend>(f: &mut Frame<B>, result: &Vec<(f64, f64)>) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(12), Constraint::Min(0)].as_ref())
        .split(f.size());
    let block = Block::default()
        .title("Shut the Box!")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    draw_boxes(f, chunks[0]);

    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
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
    f.render_widget(chart, chunks[1]);
}

fn draw_boxes<B: Backend>(f: &mut Frame<B>, area: Rect) {
    // let vert = Layout::default()
    //     .constraints([Constraint::Min(1), Constraint::Min(5), Constraint::Max(1)].as_ref())
    //     .split(area);
    let nboxes = 7;
    let pct = (100.0 / nboxes as f64).floor() as u16;
    // let constraints = vec![Constraint::Percentage(pct); nboxes];
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
