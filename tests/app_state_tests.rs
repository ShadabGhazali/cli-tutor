use cli_tutor::app::{App, ContentView, SubmitState};
use cli_tutor::config::Config;
use cli_tutor::content::load_modules;

fn make_app() -> App {
    App::new(load_modules(), Config::default())
}

// --- Input editing ---

#[test]
fn input_push_appends_to_empty() {
    let mut app = make_app();
    app.input_push('a');
    app.input_push('b');
    app.input_push('c');
    assert_eq!(app.input, "abc");
    assert_eq!(app.cursor_pos, 3);
}

#[test]
fn input_backspace_removes_before_cursor() {
    let mut app = make_app();
    "hello".chars().for_each(|c| app.input_push(c));
    app.input_backspace();
    assert_eq!(app.input, "hell");
    assert_eq!(app.cursor_pos, 4);
}

#[test]
fn input_backspace_at_start_does_nothing() {
    let mut app = make_app();
    app.input_push('a');
    app.cursor_home();
    app.input_backspace();
    assert_eq!(app.input, "a");
    assert_eq!(app.cursor_pos, 0);
}

#[test]
fn input_delete_removes_at_cursor() {
    let mut app = make_app();
    "hello".chars().for_each(|c| app.input_push(c));
    app.cursor_home();
    app.input_delete();
    assert_eq!(app.input, "ello");
    assert_eq!(app.cursor_pos, 0);
}

#[test]
fn input_delete_at_end_does_nothing() {
    let mut app = make_app();
    "hi".chars().for_each(|c| app.input_push(c));
    app.input_delete();
    assert_eq!(app.input, "hi");
}

#[test]
fn cursor_left_right_move_correctly() {
    let mut app = make_app();
    "abc".chars().for_each(|c| app.input_push(c));
    assert_eq!(app.cursor_pos, 3);
    app.cursor_left();
    assert_eq!(app.cursor_pos, 2);
    app.cursor_right();
    assert_eq!(app.cursor_pos, 3);
}

#[test]
fn cursor_home_end() {
    let mut app = make_app();
    "hello".chars().for_each(|c| app.input_push(c));
    app.cursor_home();
    assert_eq!(app.cursor_pos, 0);
    app.cursor_end();
    assert_eq!(app.cursor_pos, 5);
}

#[test]
fn cursor_left_does_not_go_below_zero() {
    let mut app = make_app();
    app.cursor_left();
    assert_eq!(app.cursor_pos, 0);
}

#[test]
fn cursor_right_does_not_exceed_input_length() {
    let mut app = make_app();
    app.input_push('x');
    app.cursor_right();
    app.cursor_right();
    assert_eq!(app.cursor_pos, 1);
}

#[test]
fn insert_at_middle_of_input() {
    let mut app = make_app();
    "ac".chars().for_each(|c| app.input_push(c));
    app.cursor_left(); // cursor between a and c
    app.input_push('b');
    assert_eq!(app.input, "abc");
}

#[test]
fn clear_input_resets_cursor() {
    let mut app = make_app();
    "hello".chars().for_each(|c| app.input_push(c));
    app.clear_input();
    assert_eq!(app.input, "");
    assert_eq!(app.cursor_pos, 0);
}

// --- View cycling ---

#[test]
fn cycle_view_goes_intro_examples_exercise() {
    let mut app = make_app();
    assert_eq!(app.current_view, ContentView::Intro);
    app.cycle_view();
    assert_eq!(app.current_view, ContentView::Examples);
    app.cycle_view();
    assert_eq!(app.current_view, ContentView::Exercise);
    app.cycle_view();
    // free_practice.VIEW.1 — FreePractice is 4th step in the cycle
    assert_eq!(app.current_view, ContentView::FreePractice);
    app.cycle_view();
    assert_eq!(app.current_view, ContentView::Intro);
}

// --- Module navigation ---

#[test]
fn select_next_module_increments() {
    let mut app = make_app();
    assert_eq!(app.selected_module, 0);
    app.select_next_module();
    assert_eq!(app.selected_module, 1);
}

#[test]
fn select_prev_module_at_start_does_nothing() {
    let mut app = make_app();
    app.select_prev_module();
    assert_eq!(app.selected_module, 0);
}

#[test]
fn select_next_module_at_end_does_nothing() {
    let mut app = make_app();
    let last = app.modules.len() - 1;
    app.selected_module = last;
    app.select_next_module();
    assert_eq!(app.selected_module, last);
}

#[test]
fn select_module_resets_view_and_exercise() {
    let mut app = make_app();
    app.cycle_view(); // go to Examples
    app.select_next_module();
    assert_eq!(app.current_view, ContentView::Intro);
    assert_eq!(app.current_exercise, 0);
}

// --- Exercise navigation ---

#[test]
fn next_exercise_increments() {
    let mut app = make_app();
    app.current_view = ContentView::Exercise;
    app.next_exercise();
    assert_eq!(app.current_exercise, 1);
}

#[test]
fn prev_exercise_at_zero_does_nothing() {
    let mut app = make_app();
    app.prev_exercise();
    assert_eq!(app.current_exercise, 0);
}

#[test]
fn next_exercise_at_last_does_nothing() {
    let mut app = make_app();
    let last = app.exercise_count() - 1;
    app.current_exercise = last;
    app.next_exercise();
    assert_eq!(app.current_exercise, last);
}

#[test]
fn next_exercise_resets_state() {
    let mut app = make_app();
    "grep mango".chars().for_each(|c| app.input_push(c));
    app.hints_revealed = 2;
    app.next_exercise();
    assert_eq!(app.input, "");
    assert_eq!(app.hints_revealed, 0);
}

// --- Hints and solution ---

#[test]
fn reveal_hint_increments_up_to_max() {
    let mut app = make_app();
    let hint_count = app.current_exercise_opt().unwrap().hints.len();
    for _ in 0..hint_count + 5 {
        app.reveal_next_hint();
    }
    assert_eq!(app.hints_revealed, hint_count);
}

#[test]
fn toggle_solution_flips() {
    let mut app = make_app();
    assert!(!app.show_solution);
    app.toggle_solution();
    assert!(app.show_solution);
    app.toggle_solution();
    assert!(!app.show_solution);
}

#[test]
fn toggle_files_flips() {
    let mut app = make_app();
    assert!(!app.show_files);
    app.toggle_files();
    assert!(app.show_files);
}

#[test]
fn toggle_help_flips() {
    let mut app = make_app();
    assert!(!app.show_help);
    app.toggle_help();
    assert!(app.show_help);
    app.toggle_help();
    assert!(!app.show_help);
}

// --- Reset ---

#[test]
fn reset_exercise_clears_all_state() {
    let mut app = make_app();
    "grep mango fruits.txt".chars().for_each(|c| app.input_push(c));
    app.hints_revealed = 1;
    app.show_solution = true;
    app.show_files = true;
    app.reset_exercise();
    assert_eq!(app.input, "");
    assert_eq!(app.hints_revealed, 0);
    assert!(!app.show_solution);
    assert!(!app.show_files);
    assert_eq!(app.submit_state, SubmitState::Idle);
}

// --- Submit ---

#[test]
fn submit_empty_command_does_nothing() {
    let mut app = make_app();
    app.submit_command();
    assert_eq!(app.submit_state, SubmitState::Idle);
    assert!(app.last_output.is_none());
}

#[test]
fn submit_wrong_command_sets_wrong_state() {
    let mut app = make_app();
    app.current_view = ContentView::Exercise;
    "echo wrong_output".chars().for_each(|c| app.input_push(c));
    app.submit_command();
    assert_eq!(app.submit_state, SubmitState::Wrong);
}

#[test]
fn submit_correct_solution_sets_correct_state() {
    let mut app = make_app();
    app.current_view = ContentView::Exercise;
    // grep.1: grep 'mango' fruits.txt
    let solution = app.current_exercise_opt().unwrap().solution.clone();
    solution.chars().for_each(|c| app.input_push(c));
    app.submit_command();
    assert_eq!(app.submit_state, SubmitState::Correct);
}

#[test]
fn submit_correct_saves_progress() {
    let mut app = make_app();
    let exercise_id = app.current_exercise_opt().unwrap().id.clone();
    let module_name = app.current_module().module.name.clone();
    let solution = app.current_exercise_opt().unwrap().solution.clone();
    solution.chars().for_each(|c| app.input_push(c));
    app.submit_command();
    assert!(app.progress.is_completed(&module_name, &exercise_id));
}

// --- Scrolling ---

#[test]
fn scroll_up_does_not_go_below_zero() {
    let mut app = make_app();
    app.current_view = ContentView::Intro;
    app.scroll_up();
    assert_eq!(app.intro_scroll, 0);
}

#[test]
fn scroll_down_increments() {
    let mut app = make_app();
    app.current_view = ContentView::Intro;
    app.scroll_down();
    assert_eq!(app.intro_scroll, 1);
}

// --- Module count sanity ---

#[test]
fn app_starts_with_fifteen_modules() {
    // more_modules.MODULES.1 — 15 modules total
    let app = make_app();
    assert_eq!(app.modules.len(), 15);
}

#[test]
fn first_module_is_grep() {
    let app = make_app();
    assert_eq!(app.current_module().module.name, "grep");
}
