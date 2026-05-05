use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

// content_display.ENG.2 — each view in its own file under src/ui/
pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = app
        .modules
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let ex_count = m.exercises.len();
            let completed = app
                .progress
                .modules
                .get(&m.module.name)
                .map(|p| p.completed.len())
                .unwrap_or(0);

            let badge = if completed == ex_count && ex_count > 0 {
                " ✓"
            } else {
                ""
            };

            let line = Line::from(vec![
                Span::styled(
                    format!("{:<10}", m.module.name),
                    if i == app.selected_module {
                        Style::default().add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    },
                ),
                Span::styled(
                    format!("[{:>2}]", ex_count),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(badge, Style::default().fg(Color::Green)),
            ]);
            ListItem::new(line)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected_module));

    let list = List::new(items)
        .block(Block::default().borders(Borders::RIGHT).title("Modules"))
        // content_display.STYLING.3 — reversed highlight for selected module
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black));

    frame.render_stateful_widget(list, area, &mut state);
}
