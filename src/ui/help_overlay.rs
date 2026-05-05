use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(_app: &App, frame: &mut Frame, area: Rect) {
    let overlay = centered_rect(60, 70, area);
    frame.render_widget(Clear, overlay);

    let lines = vec![
        Line::from(Span::styled(
            "Keyboard Reference",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Line::from(""),
        key_line("↑ / ↓", "Navigate module list"),
        key_line("Tab", "Cycle view: Intro → Examples → Exercise"),
        key_line("→ / n", "Next exercise"),
        key_line("← / p", "Previous exercise"),
        key_line("Enter", "Submit command"),
        key_line("h", "Reveal next hint"),
        key_line("s", "Show/hide solution"),
        key_line("f", "Toggle file viewer"),
        key_line("r", "Reset exercise"),
        key_line("Ctrl+L", "Clear output panel"),
        key_line("?", "Toggle this help overlay"),
        key_line("q / Ctrl+C", "Quit"),
        Line::from(""),
        Line::from(Span::styled(
            "Press ? or Esc to close",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let para = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Help ")
            .style(Style::default().bg(Color::Black)),
    );

    frame.render_widget(para, overlay);
}

fn key_line(key: &str, desc: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("  {:<15}", key),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(desc.to_string()),
    ])
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vert[1])[1]
}
