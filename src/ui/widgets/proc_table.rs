use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::app::app::App;

pub fn draw_table(f: &mut Frame, app: &mut App, area: Rect) {
    let header_cells = ["PID", "Name", "CPU %", "RAM %"].iter().map(|h| {
        Cell::from(*h).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    });

    let table_header = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray))
        .height(1)
        .bottom_margin(1);

    //Data row
    let processes = &app.processes;
    let rows: Vec<Row> = processes
        .iter()
        .map(|p| {
            let mem_mb = p.mem as f64 / 1_048_576.0;

            Row::new(vec![
                Cell::from(p.pid.clone()),
                Cell::from(p.name.clone()),
                Cell::from(format!("{:.1} %", p.cpu)),
                Cell::from(format!("{:.1} MB", mem_mb)),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Min(20),
            Constraint::Length(10),
            Constraint::Length(15),
        ],
    )
    .header(table_header)
    .block(Block::default().title(" Processes ").borders(Borders::ALL))
    .highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ");

    f.render_stateful_widget(table, area, &mut app.table_state);
}
