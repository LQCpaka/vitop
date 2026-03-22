use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
};

use crate::app::app::App;

pub fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // 50% on the left
            Constraint::Percentage(50), // 50% on the right
        ])
        .split(area);

    // ram spec
    let mem = app.memory();

    // CPU draw
    let cpu_gauge = Gauge::default()
        .block(Block::default().title(" CPU ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(app.cpu_usage as u16)
        .label(format!("{:.1} %", app.cpu_usage));

    f.render_widget(cpu_gauge, header_chunks[0]); // put into the left side

    // RAM draw
    let ram_ratio = if app.ram_total > 0 {
        app.ram_used as f64 / app.ram_total as f64
    } else {
        0.0
    };

    let ram_gauge = Gauge::default()
        .block(Block::default().title(" RAM ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(ram_ratio)
        .label(format!(
            "{:.2} GB / {:.2} GB ({:.1}%)",
            mem.ram_used_gb, mem.ram_total_gb, mem.ram_used_percent
        ));

    f.render_widget(ram_gauge, header_chunks[1]); // put into the right side
}
