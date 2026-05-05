use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let before_cursor = &app.input[..app.cursor_pos];
    let at_cursor = app.input[app.cursor_pos..].chars().next();
    let after_cursor = if let Some(c) = at_cursor {
        &app.input[app.cursor_pos + c.len_utf8()..]
    } else {
        ""
    };

    let cursor_char = at_cursor.map(|c| c.to_string()).unwrap_or(" ".to_string());

    let line = Line::from(vec![
        Span::styled(
            "$ ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(before_cursor.to_string()),
        // cursor block
        Span::styled(
            cursor_char,
            Style::default().bg(Color::White).fg(Color::Black),
        ),
        Span::raw(after_cursor.to_string()),
    ]);

    let block = Block::default().borders(Borders::ALL).title("Command");

    frame.render_widget(Paragraph::new(line).block(block), area);
}
