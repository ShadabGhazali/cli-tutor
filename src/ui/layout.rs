use crate::app::{App, ContentView};
use crate::ui::{command_list, content_pane, help_overlay};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// content_display.LAYOUT.1-5
pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();

    // content_display.LAYOUT.4 — enforce minimum terminal size
    if area.width < 80 || area.height < 24 {
        render_resize_prompt(frame, area);
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // header
            Constraint::Min(0),    // main content
            Constraint::Length(1), // footer
        ])
        .split(area);

    render_header(app, frame, rows[0]);
    render_main(app, frame, rows[1]);
    render_footer(app, frame, rows[2]);

    if app.show_help {
        help_overlay::render(app, frame, area);
    }
}

fn render_header(app: &App, frame: &mut Frame, area: Rect) {
    // content_display.LAYOUT.2
    let module = app.current_module();
    let ex_count = app.exercise_count();
    let progress_text = if ex_count > 0 {
        let completed = app
            .module_progress()
            .map(|p| p.completed.len())
            .unwrap_or(0);
        format!(
            "  {} — {}  ({}/{})",
            module.module.name, module.module.description, completed, ex_count
        )
    } else {
        format!("  {} — {}", module.module.name, module.module.description)
    };

    let view_label = match app.current_view {
        ContentView::Intro => "[Intro]",
        ContentView::Examples => "[Examples]",
        ContentView::Exercise => "[Exercise]",
    };

    let title = Line::from(vec![
        Span::styled(
            " CLI Tutor ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(&progress_text, Style::default().fg(Color::White)),
        Span::styled("  ", Style::default()),
        Span::styled(view_label, Style::default().fg(Color::Yellow)),
        Span::styled("  [?] Help", Style::default().fg(Color::DarkGray)),
    ]);

    frame.render_widget(
        Paragraph::new(title).style(Style::default().bg(Color::DarkGray)),
        area,
    );
}

fn render_main(app: &App, frame: &mut Frame, area: Rect) {
    // content_display.LAYOUT.1 — 25% / 75% split
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(area);

    command_list::render(app, frame, cols[0]);
    content_pane::render(app, frame, cols[1]);
}

fn render_footer(app: &App, frame: &mut Frame, area: Rect) {
    // content_display.LAYOUT.3
    let hints = match app.current_view {
        ContentView::Intro => " ↑↓ Scroll  Tab: Next view  ↑↓ Module  q: Quit",
        ContentView::Examples => " ↑↓ Scroll  Tab: Next view  ↑↓ Module  q: Quit",
        ContentView::Exercise => {
            " Enter: Submit  h: Hint  s: Solution  f: Files  r: Reset  Tab: View  q: Quit"
        }
    };

    frame.render_widget(
        Paragraph::new(hints).style(Style::default().bg(Color::DarkGray).fg(Color::White)),
        area,
    );
}

// content_display.LAYOUT.4
fn render_resize_prompt(frame: &mut Frame, area: Rect) {
    let msg = Paragraph::new("Please resize your terminal (min 80×24)")
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    let center = centered_rect(50, 20, area);
    frame.render_widget(msg, center);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let layout = Layout::default()
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
        .split(layout[1])[1]
}
