// free_practice.VIEW.1 — sandbox view with no expected-output matching
use crate::app::{App, SubmitState};
use crate::ui::input_bar;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let nc = app.config.no_color;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header info
            Constraint::Length(3), // input bar
            Constraint::Min(4),    // output
        ])
        .split(area);

    // free_practice.VIEW.2 — informational header
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            "Free Practice",
            crate::ui::s(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
                nc,
            ),
        ),
        Span::raw("  — type any command and press Enter. No expected output is checked."),
    ]))
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    // free_practice.VIEW.3 — shared input bar
    input_bar::render(app, frame, chunks[1]);

    // free_practice.VIEW.4 — output panel
    let (output_text, output_style) = match app.submit_state {
        SubmitState::Error => (
            app.last_output
                .as_ref()
                .map(|o| format!("Error: {}", o.stderr))
                .unwrap_or_else(|| "Error".to_string()),
            crate::ui::s(Style::default().fg(Color::Yellow), nc),
        ),
        _ => {
            let text = app
                .last_output
                .as_ref()
                .map(|o| {
                    if o.timed_out {
                        "Command timed out after 3s".to_string()
                    } else if !o.stderr.is_empty() && o.stdout.is_empty() {
                        format!("stderr: {}", o.stderr)
                    } else {
                        o.stdout.clone()
                    }
                })
                .unwrap_or_default();
            (text, Style::default())
        }
    };

    let output_para = Paragraph::new(output_text)
        .block(Block::default().borders(Borders::ALL).title("Output"))
        .style(output_style)
        .wrap(Wrap { trim: false })
        .scroll((app.output_scroll, 0));
    frame.render_widget(output_para, chunks[2]);
}
