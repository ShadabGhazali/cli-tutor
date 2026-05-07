use crate::app::{App, ContentView, DifficultyFilter};
use crate::ui::{command_list, content_pane, help_overlay, progress_overlay};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();

    if area.width < 80 || area.height < 24 {
        render_resize_prompt(frame, area);
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(app, frame, rows[0]);
    render_main(app, frame, rows[1]);
    render_footer(app, frame, rows[2]);

    if app.show_help {
        help_overlay::render(app, frame, area);
    }
    // progress_summary.OVERLAY.1 — rendered on top of everything
    if app.show_progress {
        progress_overlay::render(app, frame, area);
    }
}

fn render_header(app: &App, frame: &mut Frame, area: Rect) {
    let nc = app.config.no_color;
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
        ContentView::FreePractice => "[Free Practice]",
    };

    // difficulty_filter.FILTER.3 — show active filter in header
    let filter_label = match app.difficulty_filter {
        DifficultyFilter::None => String::new(),
        f => format!("  [{}]", f),
    };

    let title = Line::from(vec![
        Span::styled(
            " CLI Tutor ",
            crate::ui::s(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
                nc,
            ),
        ),
        Span::styled(&progress_text, crate::ui::s(Style::default().fg(Color::White), nc)),
        Span::styled("  ", Style::default()),
        Span::styled(
            view_label,
            crate::ui::s(Style::default().fg(Color::Yellow), nc),
        ),
        Span::styled(
            filter_label,
            crate::ui::s(Style::default().fg(Color::Magenta), nc),
        ),
        Span::styled(
            "  [?] Help  [P] Progress",
            crate::ui::s(Style::default().fg(Color::DarkGray), nc),
        ),
    ]);

    frame.render_widget(
        Paragraph::new(title)
            .style(crate::ui::s(Style::default().bg(Color::DarkGray), nc)),
        area,
    );
}

fn render_main(app: &App, frame: &mut Frame, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(area);

    command_list::render(app, frame, cols[0]);
    content_pane::render(app, frame, cols[1]);
}

fn render_footer(app: &App, frame: &mut Frame, area: Rect) {
    let nc = app.config.no_color;
    let hints = match app.current_view {
        ContentView::Intro | ContentView::Examples => {
            " ↑↓: Module  Tab: Next view  ^U/^D: Scroll  /: Search  d: Filter  q: Quit"
        }
        ContentView::Exercise => {
            " Enter: Submit  ↑↓: History  ^U/^D: Scroll  ^T: Hint  ^S: Solution  ^N/^P: Next/Prev  ^R: Reset  Esc: Back"
        }
        ContentView::FreePractice => {
            " Enter: Run  ↑↓: History  ^U/^D: Scroll  ^L: Clear  Tab/Esc: Back"
        }
    };

    frame.render_widget(
        Paragraph::new(hints)
            .style(crate::ui::s(Style::default().bg(Color::DarkGray).fg(Color::White), nc)),
        area,
    );
}

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
