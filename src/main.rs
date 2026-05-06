mod app;
mod content;
mod executor;
mod matcher;
mod progress;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> Result<()> {
    if std::env::args().any(|a| a == "--version" || a == "-V") {
        println!("cli-tutor {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let modules = content::load_modules();
    let mut app = App::new(modules);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        // Render only on event (event-driven, no tick loop)
        terminal.draw(|f| ui::layout::render(app, f))?;

        // Block until next event — near-zero CPU at idle
        let ev = event::read()?;

        match ev {
            // content_display.LAYOUT.5 — handle resize
            Event::Resize(_, _) => {}
            Event::Key(key) => handle_key(app, key),
            _ => {}
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) {
    use app::ContentView;

    // Ctrl+C quits everywhere
    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
        app.should_quit = true;
        return;
    }

    if app.show_help {
        if matches!(key.code, KeyCode::Esc) || key.code == KeyCode::Char('?') {
            app.toggle_help();
        }
        return;
    }

    match app.current_view {
        ContentView::Intro | ContentView::Examples => handle_browse_keys(app, key),
        ContentView::Exercise => handle_exercise_keys(app, key),
    }
}

fn handle_browse_keys(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        (KeyModifiers::NONE, KeyCode::Char('q')) => app.should_quit = true,
        (KeyModifiers::NONE, KeyCode::Char('?')) => app.toggle_help(),
        (KeyModifiers::NONE, KeyCode::Tab) => app.cycle_view(),
        (KeyModifiers::NONE, KeyCode::Up) => app.select_prev_module(),
        (KeyModifiers::NONE, KeyCode::Down) => app.select_next_module(),
        (KeyModifiers::NONE, KeyCode::PageUp) => app.scroll_up(),
        (KeyModifiers::NONE, KeyCode::PageDown) => app.scroll_down(),
        _ => {}
    }
}

fn handle_exercise_keys(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        // Ctrl shortcuts — safe to use while typing
        (KeyModifiers::CONTROL, KeyCode::Char('n')) => app.next_exercise(),
        (KeyModifiers::CONTROL, KeyCode::Char('p')) => app.prev_exercise(),
        (KeyModifiers::CONTROL, KeyCode::Char('t')) => app.reveal_next_hint(),
        (KeyModifiers::CONTROL, KeyCode::Char('s')) => app.toggle_solution(),
        (KeyModifiers::CONTROL, KeyCode::Char('f')) => app.toggle_files(),
        (KeyModifiers::CONTROL, KeyCode::Char('r')) => app.reset_exercise(),
        (KeyModifiers::CONTROL, KeyCode::Char('l')) => app.clear_output(),

        // View / help
        (KeyModifiers::NONE, KeyCode::Tab) => app.cycle_view(),
        (KeyModifiers::NONE, KeyCode::Esc) => app.cycle_view(),

        // Submit
        (KeyModifiers::NONE, KeyCode::Enter) => app.submit_command(),

        // Scroll output panel
        (KeyModifiers::NONE, KeyCode::Up) => app.scroll_up(),
        (KeyModifiers::NONE, KeyCode::Down) => app.scroll_down(),

        // All printable characters go to input — no letter shortcuts here
        (KeyModifiers::NONE, KeyCode::Char(c)) => app.input_push(c),
        (KeyModifiers::SHIFT, KeyCode::Char(c)) => app.input_push(c),

        // Cursor / editing
        (KeyModifiers::NONE, KeyCode::Backspace) => app.input_backspace(),
        (KeyModifiers::NONE, KeyCode::Delete) => app.input_delete(),
        (KeyModifiers::NONE, KeyCode::Left) => app.cursor_left(),
        (KeyModifiers::NONE, KeyCode::Right) => app.cursor_right(),
        (KeyModifiers::NONE, KeyCode::Home) => app.cursor_home(),
        (KeyModifiers::NONE, KeyCode::End) => app.cursor_end(),

        _ => {}
    }
}
