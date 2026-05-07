pub mod command_list;
pub mod content_pane;
pub mod exercise_view;
pub mod free_practice_view;
pub mod help_overlay;
pub mod input_bar;
pub mod layout;
pub mod progress_overlay;

/// no_color.DISPLAY.1 — strip all styling when no_color is enabled
pub fn s(style: ratatui::style::Style, no_color: bool) -> ratatui::style::Style {
    if no_color {
        ratatui::style::Style::reset()
    } else {
        style
    }
}
