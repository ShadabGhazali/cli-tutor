use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let nc = app.config.no_color;

    // module_search.SEARCH.2 — split area to show search bar when active
    let (search_area, list_area) = if app.search_active {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(area);
        (Some(chunks[0]), chunks[1])
    } else {
        (None, area)
    };

    // module_search.SEARCH.2 — render search bar
    if let Some(sa) = search_area {
        let search_line = Line::from(vec![
            Span::styled(
                "/ ",
                crate::ui::s(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD), nc),
            ),
            Span::raw(app.search_query.clone()),
            Span::styled(
                " ",
                crate::ui::s(Style::default().bg(Color::White).fg(Color::Black), nc),
            ),
        ]);
        frame.render_widget(Paragraph::new(search_line), sa);
    }

    let visible = app.visible_module_indices();

    let items: Vec<ListItem> = visible
        .iter()
        .map(|&i| {
            let m = &app.modules[i];
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

            let is_selected = i == app.selected_module;

            let line = Line::from(vec![
                Span::styled(
                    format!("{:<10}", m.module.name),
                    if is_selected {
                        crate::ui::s(Style::default().add_modifier(Modifier::BOLD), nc)
                    } else {
                        Style::default()
                    },
                ),
                Span::styled(
                    format!("[{:>2}]", ex_count),
                    crate::ui::s(Style::default().fg(Color::DarkGray), nc),
                ),
                Span::styled(
                    badge,
                    crate::ui::s(Style::default().fg(Color::Green), nc),
                ),
            ]);
            ListItem::new(line)
        })
        .collect();

    // Find position of selected_module within the visible list for ListState
    let selected_pos = visible.iter().position(|&i| i == app.selected_module);
    let mut state = ListState::default();
    state.select(selected_pos);

    let list = List::new(items)
        .block(Block::default().borders(Borders::RIGHT).title("Modules"))
        .highlight_style(crate::ui::s(
            Style::default().bg(Color::White).fg(Color::Black),
            nc,
        ));

    frame.render_stateful_widget(list, list_area, &mut state);
}
