use crate::app::{App, SubmitState};
use crate::content::types::Difficulty;
use crate::ui::input_bar;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

// content_display.EXERCISE_VIEW — all requirements annotated inline
pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let exercise = match app.current_exercise_opt() {
        Some(ex) => ex,
        None => {
            frame.render_widget(
                Paragraph::new("No exercises in this module.")
                    .block(Block::default().borders(Borders::ALL).title("Exercise")),
                area,
            );
            return;
        }
    };

    let module_name = &app.current_module().module.name;
    let ex_count = app.exercise_count();

    // content_display.EXERCISE_VIEW.6 — difficulty badge
    let (diff_label, diff_color) = match exercise.difficulty {
        Difficulty::Beginner => ("Beginner", Color::Green),
        Difficulty::Intermediate => ("Intermediate", Color::Yellow),
        Difficulty::Advanced => ("Advanced", Color::Red),
    };

    let done_mark = if app.exercise_is_completed() {
        " ✓"
    } else {
        ""
    };
    let title = format!(
        " Exercise {}/{}{} — {} ",
        app.current_exercise + 1,
        ex_count,
        done_mark,
        diff_label
    );

    // Layout: question | [files] | [hints] | input | output
    let has_files = app.show_files && !exercise.fixtures.is_empty();
    let has_hints = app.hints_revealed > 0;

    let mut constraints = vec![Constraint::Length(question_height(&exercise.question))];
    if exercise.fixtures.is_empty() {
        // no file toggle row
    } else {
        constraints.push(Constraint::Length(1)); // files toggle hint
        if has_files {
            constraints.push(Constraint::Length(
                (exercise.fixtures.len() * 3).min(12) as u16
            ));
        }
    }
    if has_hints {
        constraints.push(Constraint::Length(app.hints_revealed as u16 + 2));
    }
    constraints.push(Constraint::Length(3)); // input bar
    constraints.push(Constraint::Min(4)); // output

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    let mut chunk_idx = 0;

    // content_display.EXERCISE_VIEW.1 — question
    let question_block = Paragraph::new(exercise.question.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(vec![
                    Span::raw(title),
                    Span::styled(
                        diff_label,
                        Style::default().fg(diff_color).add_modifier(Modifier::BOLD),
                    ),
                ])),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(question_block, chunks[chunk_idx]);
    chunk_idx += 1;

    // content_display.EXERCISE_VIEW.2 — files section
    if !exercise.fixtures.is_empty() {
        let toggle_hint = if app.show_files {
            "f: hide files"
        } else {
            "f: show files"
        };
        frame.render_widget(
            Paragraph::new(toggle_hint).style(Style::default().fg(Color::DarkGray)),
            chunks[chunk_idx],
        );
        chunk_idx += 1;

        if has_files {
            let mut file_lines: Vec<Line<'static>> = Vec::new();
            for fixture in &exercise.fixtures {
                file_lines.push(Line::from(Span::styled(
                    format!("── {} ──", fixture.filename),
                    Style::default().fg(Color::Cyan),
                )));
                for (i, l) in fixture.content.lines().enumerate().take(20) {
                    file_lines.push(Line::from(Span::styled(
                        format!("{:>3}  {}", i + 1, l),
                        Style::default().fg(Color::DarkGray),
                    )));
                }
            }
            frame.render_widget(
                Paragraph::new(file_lines)
                    .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
                chunks[chunk_idx],
            );
            chunk_idx += 1;
        }
    }

    // content_display.EXERCISE_VIEW.7 — hints block
    if has_hints {
        let mut hint_lines: Vec<Line<'static>> = Vec::new();
        for i in 0..app.hints_revealed {
            if let Some(hint) = exercise.hints.get(i) {
                hint_lines.push(Line::from(Span::styled(
                    format!("Hint {}: {}", i + 1, hint),
                    Style::default().fg(Color::Cyan),
                )));
            }
        }
        if app.show_solution {
            hint_lines.push(Line::from(Span::styled(
                format!("Solution: {}", exercise.solution),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )));
        }
        frame.render_widget(
            Paragraph::new(hint_lines).block(Block::default().borders(Borders::ALL).title("Hints")),
            chunks[chunk_idx],
        );
        chunk_idx += 1;
    }

    // content_display.EXERCISE_VIEW.3 — input bar
    input_bar::render(app, frame, chunks[chunk_idx]);
    chunk_idx += 1;

    // content_display.EXERCISE_VIEW.4,5 — output panel, scrollable
    let (output_text, output_style) = match app.submit_state {
        SubmitState::Correct => (
            "✓ Correct!\n".to_string()
                + app
                    .last_output
                    .as_ref()
                    .map(|o| o.stdout.as_str())
                    .unwrap_or(""),
            Style::default().fg(Color::Green),
        ),
        SubmitState::Wrong => {
            let out = app.last_output.as_ref();
            let stdout = out.map(|o| o.stdout.as_str()).unwrap_or("");
            let stderr = out.map(|o| o.stderr.as_str()).unwrap_or("");
            let timed_out = out.map(|o| o.timed_out).unwrap_or(false);
            let text = if timed_out {
                "✗ Command timed out after 3s".to_string()
            } else if !stderr.is_empty() {
                format!("✗ Wrong\nstderr: {stderr}")
            } else {
                format!("✗ Wrong\nGot:\n{stdout}")
            };
            (text, Style::default().fg(Color::Red))
        }
        SubmitState::Error => (
            app.last_output
                .as_ref()
                .map(|o| format!("Error: {}", o.stderr))
                .unwrap_or_else(|| "Error".to_string()),
            Style::default().fg(Color::Yellow),
        ),
        SubmitState::Idle => {
            let text = app
                .last_output
                .as_ref()
                .map(|o| o.stdout.clone())
                .unwrap_or_default();
            (text, Style::default())
        }
    };

    let output_para = Paragraph::new(output_text)
        .block(Block::default().borders(Borders::ALL).title("Output"))
        .style(output_style)
        .wrap(Wrap { trim: false })
        .scroll((app.output_scroll, 0));
    frame.render_widget(output_para, chunks[chunk_idx]);

    let nav_area = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };
    let nav = format!(
        " [← p] Exercise {}/{} [n →]   {} ",
        app.current_exercise + 1,
        ex_count,
        module_name
    );
    frame.render_widget(
        Paragraph::new(nav).style(Style::default().fg(Color::DarkGray)),
        nav_area,
    );
}

fn question_height(question: &str) -> u16 {
    let lines = question.lines().count() as u16;
    (lines + 2).clamp(4, 10)
}
