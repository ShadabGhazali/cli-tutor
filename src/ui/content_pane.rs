use crate::app::{App, ContentView};
use crate::ui::exercise_view;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

// content_display.ENG.2 — each view in its own file
pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    match app.current_view {
        ContentView::Intro => render_intro(app, frame, area),
        ContentView::Examples => render_examples(app, frame, area),
        ContentView::Exercise => exercise_view::render(app, frame, area),
    }
}

// content_display.INTRO_VIEW
fn render_intro(app: &App, frame: &mut Frame, area: Rect) {
    let text = &app.current_module().intro.text;
    let lines = render_intro_text(text);

    // content_display.INTRO_VIEW.1 — scrollable paragraph
    let para = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                // content_display.INTRO_VIEW.4
                .title("Introduction"),
        )
        .wrap(Wrap { trim: false })
        .scroll((app.intro_scroll, 0));

    frame.render_widget(para, area);
}

fn render_intro_text(text: &str) -> Vec<Line<'static>> {
    text.lines()
        .map(|line| {
            let owned = line.to_string();
            // content_display.INTRO_VIEW.3 — headings bold + underlined
            if owned.starts_with("## ") {
                return Line::from(Span::styled(
                    owned,
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::UNDERLINED),
                ));
            }
            // content_display.INTRO_VIEW.2 — inline code with distinct style
            if owned.contains('`') {
                return Line::from(render_inline_code(owned));
            }
            Line::from(Span::raw(owned))
        })
        .collect()
}

fn render_inline_code(line: String) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut remaining = line.as_str();
    let mut in_code = false;

    while let Some(pos) = remaining.find('`') {
        let before = remaining[..pos].to_string();
        if !before.is_empty() {
            spans.push(if in_code {
                Span::styled(
                    before,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw(before)
            });
        }
        in_code = !in_code;
        remaining = &remaining[pos + 1..];
    }
    if !remaining.is_empty() {
        spans.push(Span::raw(remaining.to_string()));
    }
    spans
}

// content_display.EXAMPLES_VIEW
fn render_examples(app: &App, frame: &mut Frame, area: Rect) {
    let module = app.current_module();
    let count = module.examples.len();

    let mut lines: Vec<Line<'static>> = Vec::new();

    for (i, ex) in module.examples.iter().enumerate() {
        // content_display.EXAMPLES_VIEW.1 — titled block
        lines.push(Line::from(Span::styled(
            ex.title.clone(),
            Style::default().add_modifier(Modifier::BOLD),
        )));
        if !ex.description.is_empty() {
            lines.push(Line::from(Span::raw(ex.description.clone())));
        }
        // content_display.EXAMPLES_VIEW.2,3,4 — command with $ prefix, bold
        lines.push(Line::from(vec![
            Span::styled("$ ", Style::default().fg(Color::Green)),
            Span::styled(
                ex.command.clone(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        for out_line in ex.output.lines() {
            lines.push(Line::from(Span::styled(
                out_line.to_string(),
                Style::default().fg(Color::DarkGray),
            )));
        }
        // content_display.EXAMPLES_VIEW.5 — divider between examples
        if i + 1 < count {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "─".repeat(40),
                Style::default().fg(Color::DarkGray),
            )));
            lines.push(Line::from(""));
        }
    }

    // content_display.EXAMPLES_VIEW.7 — count in title
    let title = format!("Examples ({})", count);
    let para = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: false })
        // content_display.EXAMPLES_VIEW.6 — scrollable list
        .scroll((app.examples_scroll, 0));

    frame.render_widget(para, area);
}
