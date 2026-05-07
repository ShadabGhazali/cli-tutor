mod app;
mod config;
mod content;
mod executor;
mod matcher;
mod progress;
mod ui;

use anyhow::Result;
use app::{App, ContentView};
use config::Config;
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
    // shell_completions.CLI.1 — handle --completions before terminal setup
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut no_color_flag = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--version" | "-V" => {
                println!("cli-tutor {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "--no-color" => {
                no_color_flag = true;
            }
            "--completions" => {
                let shell = args.get(i + 1).map(|s| s.as_str()).unwrap_or("");
                print_completions(shell);
                return Ok(());
            }
            _ => {}
        }
        i += 1;
    }

    // config_file.CONFIG.3
    let mut config = Config::load();
    // no_color.CLI.1 — CLI flag overrides config file
    if no_color_flag {
        config.no_color = true;
    }

    let modules = content::load_modules();
    let mut app = App::new(modules, config);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        // paste_support.INPUT.1 — enable bracketed paste
        crossterm::event::EnableBracketedPaste
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        // paste_support.INPUT.1 — disable bracketed paste on exit
        crossterm::event::DisableBracketedPaste
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::layout::render(app, f))?;

        let ev = event::read()?;

        match ev {
            Event::Resize(_, _) => {}
            Event::Key(key) => handle_key(app, key),
            // paste_support.INPUT.2 — insert pasted text into input
            Event::Paste(s) => {
                if matches!(
                    app.current_view,
                    ContentView::Exercise | ContentView::FreePractice
                ) {
                    app.input_paste(&s);
                }
            }
            _ => {}
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) {
    // Ctrl+C quits everywhere
    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
        app.should_quit = true;
        return;
    }

    // progress_summary.OVERLAY.4 — P toggles progress from anywhere
    if key.modifiers == KeyModifiers::SHIFT && key.code == KeyCode::Char('P') {
        app.toggle_progress();
        return;
    }

    // progress overlay captures all keys except P
    if app.show_progress {
        if key.modifiers == KeyModifiers::SHIFT && key.code == KeyCode::Char('P') {
            app.toggle_progress();
        }
        return;
    }

    if app.show_help {
        if matches!(key.code, KeyCode::Esc) || key.code == KeyCode::Char('?') {
            app.toggle_help();
        }
        return;
    }

    // module_search.SEARCH.1 — search bar intercepts keys when active
    if app.search_active {
        handle_search_keys(app, key);
        return;
    }

    match app.current_view {
        ContentView::Intro | ContentView::Examples => handle_browse_keys(app, key),
        ContentView::Exercise => handle_exercise_keys(app, key),
        ContentView::FreePractice => handle_free_practice_keys(app, key),
    }
}

fn handle_search_keys(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        (KeyModifiers::NONE, KeyCode::Esc) => app.search_cancel(),
        (KeyModifiers::NONE, KeyCode::Enter) | (KeyModifiers::NONE, KeyCode::Tab) => {
            app.search_confirm()
        }
        (KeyModifiers::NONE, KeyCode::Backspace) => app.search_backspace(),
        (KeyModifiers::NONE, KeyCode::Char(c)) => app.search_push(c),
        _ => {}
    }
}

fn handle_browse_keys(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        (KeyModifiers::NONE, KeyCode::Char('q')) => app.should_quit = true,
        (KeyModifiers::NONE, KeyCode::Char('?')) => app.toggle_help(),
        // module_search.SEARCH.1 — / activates search
        (KeyModifiers::NONE, KeyCode::Char('/')) => app.activate_search(),
        // difficulty_filter.FILTER.2 — d cycles filter
        (KeyModifiers::NONE, KeyCode::Char('d')) => app.cycle_difficulty_filter(),
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
        // Ctrl shortcuts
        (KeyModifiers::CONTROL, KeyCode::Char('n')) => app.next_exercise(),
        (KeyModifiers::CONTROL, KeyCode::Char('p')) => app.prev_exercise(),
        (KeyModifiers::CONTROL, KeyCode::Char('t')) => app.reveal_next_hint(),
        (KeyModifiers::CONTROL, KeyCode::Char('s')) => app.toggle_solution(),
        (KeyModifiers::CONTROL, KeyCode::Char('f')) => app.toggle_files(),
        (KeyModifiers::CONTROL, KeyCode::Char('r')) => app.reset_exercise(),
        (KeyModifiers::CONTROL, KeyCode::Char('l')) => app.clear_output(),
        // word_jump.CURSOR.1,2
        (KeyModifiers::CONTROL, KeyCode::Left) => app.cursor_word_left(),
        (KeyModifiers::CONTROL, KeyCode::Right) => app.cursor_word_right(),

        (KeyModifiers::NONE, KeyCode::Tab) => app.cycle_view(),
        (KeyModifiers::NONE, KeyCode::Esc) => app.cycle_view(),
        (KeyModifiers::NONE, KeyCode::Enter) => app.submit_command(),

        // command_history.HISTORY.3 — ↑/↓ navigate history
        (KeyModifiers::NONE, KeyCode::Up) => app.history_prev(),
        (KeyModifiers::NONE, KeyCode::Down) => app.history_next(),
        // PgUp/PgDn for output scroll
        (KeyModifiers::NONE, KeyCode::PageUp) => app.scroll_up(),
        (KeyModifiers::NONE, KeyCode::PageDown) => app.scroll_down(),

        // All printable characters go to input
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

fn handle_free_practice_keys(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        (KeyModifiers::CONTROL, KeyCode::Char('l')) => app.clear_output(),
        (KeyModifiers::CONTROL, KeyCode::Left) => app.cursor_word_left(),
        (KeyModifiers::CONTROL, KeyCode::Right) => app.cursor_word_right(),

        (KeyModifiers::NONE, KeyCode::Tab) => app.cycle_view(),
        (KeyModifiers::NONE, KeyCode::Esc) => app.cycle_view(),
        (KeyModifiers::NONE, KeyCode::Enter) => app.submit_command_free(),

        (KeyModifiers::NONE, KeyCode::Up) => app.history_prev(),
        (KeyModifiers::NONE, KeyCode::Down) => app.history_next(),
        (KeyModifiers::NONE, KeyCode::PageUp) => app.scroll_up(),
        (KeyModifiers::NONE, KeyCode::PageDown) => app.scroll_down(),

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

// shell_completions.CLI.2 — static completion scripts for bash, zsh, fish
fn print_completions(shell: &str) {
    match shell {
        "bash" => print!(
            r#"# cli-tutor bash completion
# Add to ~/.bash_completion or source from ~/.bashrc
_cli_tutor() {{
    local cur prev
    cur="${{COMP_WORDS[COMP_CWORD]}}"
    prev="${{COMP_WORDS[COMP_CWORD-1]}}"
    case "$prev" in
        --completions)
            COMPREPLY=($(compgen -W "bash zsh fish" -- "$cur"))
            return ;;
    esac
    COMPREPLY=($(compgen -W "--version --no-color --completions" -- "$cur"))
}}
complete -F _cli_tutor cli-tutor
"#
        ),
        "zsh" => print!(
            r#"#compdef cli-tutor
# cli-tutor zsh completion
# Place in a directory listed in $fpath, e.g. ~/.zsh/completions/_cli-tutor
_cli_tutor() {{
    local -a opts
    opts=(
        '--version[Print version]'
        '--no-color[Disable color output]'
        '--completions[Generate shell completions]:shell:(bash zsh fish)'
    )
    _arguments $opts
}}
_cli_tutor "$@"
"#
        ),
        "fish" => print!(
            r#"# cli-tutor fish completion
# Place in ~/.config/fish/completions/cli-tutor.fish
complete -c cli-tutor -l version -d 'Print version'
complete -c cli-tutor -l no-color -d 'Disable color output'
complete -c cli-tutor -l completions -d 'Generate shell completions' -r -a 'bash zsh fish'
"#
        ),
        _ => {
            eprintln!("Unknown shell '{}'. Supported: bash, zsh, fish", shell);
            std::process::exit(1);
        }
    }
}
