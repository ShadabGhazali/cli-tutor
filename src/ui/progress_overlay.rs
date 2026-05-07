// progress_summary.OVERLAY.1 — modal showing per-module completion and best times
use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let overlay = centered_rect(70, 80, area);
    frame.render_widget(Clear, overlay);

    let nc = app.config.no_color;

    // progress_summary.OVERLAY.2 — per-module rows
    let mut lines: Vec<Line<'static>> = vec![
        Line::from(Span::styled(
            "Progress Summary",
            crate::ui::s(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
                nc,
            ),
        )),
        Line::from(""),
    ];

    let show_times = app.config.timed_challenge;
    let header = if show_times {
        "  Module          Completed    Best Time"
    } else {
        "  Module          Completed"
    };
    lines.push(Line::from(Span::styled(
        header,
        crate::ui::s(
            Style::default().add_modifier(Modifier::UNDERLINED),
            nc,
        ),
    )));
    lines.push(Line::from(""));

    let mut total_completed = 0usize;
    let mut total_exercises = 0usize;

    for m in &app.modules {
        let ex_count = m.exercises.len();
        let completed = app
            .progress
            .modules
            .get(&m.module.name)
            .map(|p| p.completed.len())
            .unwrap_or(0);
        total_completed += completed;
        total_exercises += ex_count;

        let pct = if ex_count > 0 {
            completed * 100 / ex_count
        } else {
            0
        };
        let bar = progress_bar(pct);

        // progress_summary.OVERLAY.4 — show best times when timed_challenge enabled
        let time_str = if show_times {
            let best = m
                .exercises
                .iter()
                .filter_map(|ex| app.progress.best_time(&m.module.name, &ex.id))
                .min();
            match best {
                Some(ms) => format!("  {:.1}s", ms as f64 / 1000.0),
                None => "  —".to_string(),
            }
        } else {
            String::new()
        };

        let done_mark = if completed == ex_count && ex_count > 0 {
            " ✓"
        } else {
            "  "
        };
        let color = if completed == ex_count && ex_count > 0 {
            Color::Green
        } else if completed > 0 {
            Color::Yellow
        } else {
            Color::DarkGray
        };

        let row = format!(
            "  {:<16}{}{}/{}  {}{}{}",
            m.module.name,
            done_mark,
            completed,
            ex_count,
            bar,
            if pct < 100 {
                format!(" {pct}%")
            } else {
                "    ".to_string()
            },
            time_str,
        );
        lines.push(Line::from(Span::styled(
            row,
            crate::ui::s(Style::default().fg(color), nc),
        )));
    }

    // progress_summary.OVERLAY.3 — totals row
    lines.push(Line::from(""));
    let total_pct = if total_exercises > 0 {
        total_completed * 100 / total_exercises
    } else {
        0
    };
    lines.push(Line::from(Span::styled(
        format!(
            "  Total             {}/{} ({total_pct}%)",
            total_completed, total_exercises
        ),
        crate::ui::s(Style::default().add_modifier(Modifier::BOLD), nc),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Press P to close",
        crate::ui::s(Style::default().fg(Color::DarkGray), nc),
    )));

    let para = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Progress ")
            .style(crate::ui::s(Style::default().bg(Color::Black), nc)),
    );
    frame.render_widget(para, overlay);
}

fn progress_bar(pct: usize) -> String {
    let filled = pct / 10; // 0-10 blocks
    let empty = 10 - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
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
