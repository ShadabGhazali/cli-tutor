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

    // Global keys
    match (key.modifiers, key.code) {
        (KeyModifiers::NONE, KeyCode::Char('q')) | (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
            app.should_quit = true;
            return;
        }
        (KeyModifiers::NONE, KeyCode::Char('?')) => {
            app.toggle_help();
            return;
        }
        _ => {}
    }

    if app.show_help {
        // Any key closes help
        if matches!(key.code, KeyCode::Char('?') | KeyCode::Esc) {
            app.toggle_help();
        }
        return;
    }

    match key.code {
        KeyCode::Tab => app.cycle_view(),
        KeyCode::Up => match app.current_view {
            ContentView::Exercise => app.scroll_up(),
            _ => {
                app.select_prev_module();
            }
        },
        KeyCode::Down => match app.current_view {
            ContentView::Exercise => app.scroll_down(),
            _ => {
                app.select_next_module();
            }
        },
        _ => {}
    }

    match app.current_view {
        ContentView::Intro | ContentView::Examples => handle_scroll_keys(app, key),
        ContentView::Exercise => handle_exercise_keys(app, key),
    }
}

fn handle_scroll_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Up => app.scroll_up(),
        KeyCode::Down => app.scroll_down(),
        _ => {}
    }
}

fn handle_exercise_keys(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        // Navigation — n/p only; Left/Right are reserved for cursor movement
        (KeyModifiers::NONE, KeyCode::Char('n')) => app.next_exercise(),
        (KeyModifiers::NONE, KeyCode::Char('p')) => app.prev_exercise(),

        // Exercise actions
        (KeyModifiers::NONE, KeyCode::Enter) => app.submit_command(),
        (KeyModifiers::NONE, KeyCode::Char('h')) => app.reveal_next_hint(),
        (KeyModifiers::NONE, KeyCode::Char('s')) => app.toggle_solution(),
        (KeyModifiers::NONE, KeyCode::Char('f')) => app.toggle_files(),
        (KeyModifiers::NONE, KeyCode::Char('r')) => app.reset_exercise(),
        (KeyModifiers::CONTROL, KeyCode::Char('l')) => app.clear_output(),

        // Text input
        (KeyModifiers::NONE, KeyCode::Char(c)) => app.input_push(c),
        (KeyModifiers::SHIFT, KeyCode::Char(c)) => app.input_push(c),
        (KeyModifiers::NONE, KeyCode::Backspace) => app.input_backspace(),
        (KeyModifiers::NONE, KeyCode::Delete) => app.input_delete(),
        (KeyModifiers::NONE, KeyCode::Left) => app.cursor_left(),
        (KeyModifiers::NONE, KeyCode::Right) => app.cursor_right(),
        (KeyModifiers::NONE, KeyCode::Home) => app.cursor_home(),
        (KeyModifiers::NONE, KeyCode::End) => app.cursor_end(),

        _ => {}
    }
}
