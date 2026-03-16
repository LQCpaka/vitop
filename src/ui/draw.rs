use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::app::App;

pub fn draw_ui(f: &mut Frame, app: &App) {
    //Vertical
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header = 3 lines
            Constraint::Min(0),    // Main = the rest place
            Constraint::Length(3), // Footer = 3 liens
        ])
        .split(f.size());

    // Header

    let ram_used_gb = app.ram_used as f64 / 1_073_741_824.0;
    let ram_total_gb = app.ram_total as f64 / 1_073_741_824.0;
    let ram_used_percent = (ram_used_gb / ram_total_gb) * 100 as f64;

    let header_text = format!(
        " CPU: {:.1}% \n RAM: {:.2} GB / {:.2} GB ({:.1} %)",
        app.cpu_usage, ram_used_gb, ram_total_gb, ram_used_percent
    );

    let header = Paragraph::new(header_text)
        .block(Block::default().title(" Hệ Thống ").borders(Borders::ALL));
    f.render_widget(header, chunks[0]); // the top header

    // Main
    let main_content = Paragraph::new(" Proccess running placehold").block(
        Block::default()
            .title(" Tiến trình (Processes) ")
            .borders(Borders::ALL),
    );
    f.render_widget(main_content, chunks[1]);

    // Footer
    let footer = Paragraph::new(" [q] Thoát | [Up/Down] Cuộn chuột")
        .block(Block::default().title(" Hướng dẫn ").borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
