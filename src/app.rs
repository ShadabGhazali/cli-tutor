use crate::content::types::{Exercise, ModuleFile};
use crate::executor::{ExecutionResult, Executor};
use crate::matcher::Matcher;
use crate::progress::{ModuleProgress, Progress};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentView {
    Intro,
    Examples,
    Exercise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubmitState {
    Idle,
    Correct,
    Wrong,
    Error,
}

pub struct App {
    pub modules: Vec<ModuleFile>,
    pub selected_module: usize,
    pub current_view: ContentView,
    pub current_exercise: usize,

    pub input: String,
    pub cursor_pos: usize,

    pub last_output: Option<ExecutionResult>,
    pub submit_state: SubmitState,

    pub hints_revealed: usize,
    pub show_solution: bool,
    pub show_files: bool,
    pub show_help: bool,

    pub intro_scroll: u16,
    pub examples_scroll: u16,
    pub output_scroll: u16,

    pub progress: Progress,
    pub should_quit: bool,
}

impl App {
    pub fn new(modules: Vec<ModuleFile>) -> Self {
        let progress = Progress::load();
        App {
            modules,
            selected_module: 0,
            current_view: ContentView::Intro,
            current_exercise: 0,
            input: String::new(),
            cursor_pos: 0,
            last_output: None,
            submit_state: SubmitState::Idle,
            hints_revealed: 0,
            show_solution: false,
            show_files: false,
            show_help: false,
            intro_scroll: 0,
            examples_scroll: 0,
            output_scroll: 0,
            progress,
            should_quit: false,
        }
    }

    pub fn current_module(&self) -> &ModuleFile {
        &self.modules[self.selected_module]
    }

    pub fn current_exercise_opt(&self) -> Option<&Exercise> {
        self.current_module().exercises.get(self.current_exercise)
    }

    pub fn exercise_count(&self) -> usize {
        self.current_module().exercises.len()
    }

    pub fn module_progress(&self) -> Option<&ModuleProgress> {
        let name = &self.current_module().module.name;
        self.progress.modules.get(name)
    }

    pub fn exercise_is_completed(&self) -> bool {
        if let Some(ex) = self.current_exercise_opt() {
            let module_name = &self.current_module().module.name;
            self.progress.is_completed(module_name, &ex.id)
        } else {
            false
        }
    }

    // --- Input handling ---

    pub fn input_push(&mut self, c: char) {
        self.input.insert(self.cursor_pos, c);
        self.cursor_pos += c.len_utf8();
    }

    pub fn input_backspace(&mut self) {
        if self.cursor_pos > 0 {
            let prev = self.input[..self.cursor_pos]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.input.remove(prev);
            self.cursor_pos = prev;
        }
    }

    pub fn input_delete(&mut self) {
        if self.cursor_pos < self.input.len() {
            self.input.remove(self.cursor_pos);
        }
    }

    pub fn cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos = self.input[..self.cursor_pos]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
    }

    pub fn cursor_right(&mut self) {
        if self.cursor_pos < self.input.len() {
            let mut chars = self.input[self.cursor_pos..].char_indices();
            if let Some((_, c)) = chars.next() {
                self.cursor_pos += c.len_utf8();
            }
        }
    }

    pub fn cursor_home(&mut self) {
        self.cursor_pos = 0;
    }

    pub fn cursor_end(&mut self) {
        self.cursor_pos = self.input.len();
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_pos = 0;
    }

    // --- Navigation ---

    pub fn select_prev_module(&mut self) {
        if self.selected_module > 0 {
            self.selected_module -= 1;
            self.reset_content_state();
        }
    }

    pub fn select_next_module(&mut self) {
        if self.selected_module + 1 < self.modules.len() {
            self.selected_module += 1;
            self.reset_content_state();
        }
    }

    pub fn cycle_view(&mut self) {
        self.current_view = match self.current_view {
            ContentView::Intro => ContentView::Examples,
            ContentView::Examples => ContentView::Exercise,
            ContentView::Exercise => ContentView::Intro,
        };
    }

    pub fn next_exercise(&mut self) {
        let count = self.exercise_count();
        if count > 0 && self.current_exercise + 1 < count {
            self.current_exercise += 1;
            self.reset_exercise_state();
        }
    }

    pub fn prev_exercise(&mut self) {
        if self.current_exercise > 0 {
            self.current_exercise -= 1;
            self.reset_exercise_state();
        }
    }

    // --- Exercise actions ---

    pub fn submit_command(&mut self) {
        let command = self.input.clone();
        if command.trim().is_empty() {
            return;
        }

        let exercise = match self.current_exercise_opt() {
            Some(ex) => ex.clone(),
            None => return,
        };

        let result = Executor::run(&command, &exercise.fixtures);
        let (exec_result, exec_error) = match result {
            Ok(o) => (Some(o), false),
            Err(e) => (
                Some(ExecutionResult {
                    stdout: String::new(),
                    stderr: e.to_string(),
                    timed_out: false,
                }),
                true,
            ),
        };

        let correct = exec_result
            .as_ref()
            .map(|o| {
                !exec_error
                    && Matcher::check(&o.stdout, &exercise.expected_output, &exercise.match_mode)
            })
            .unwrap_or(false);

        self.last_output = exec_result;
        self.output_scroll = 0;

        let module_name = self.current_module().module.name.clone();
        if exec_error {
            self.submit_state = SubmitState::Error;
        } else if correct {
            self.submit_state = SubmitState::Correct;
            self.progress.mark_completed(&module_name, &exercise.id);
            let _ = self.progress.save();
        } else {
            self.submit_state = SubmitState::Wrong;
            self.progress.mark_attempted(&module_name, &exercise.id);
        }
    }

    pub fn reveal_next_hint(&mut self) {
        if let Some(ex) = self.current_exercise_opt() {
            if self.hints_revealed < ex.hints.len() {
                self.hints_revealed += 1;
            }
        }
    }

    pub fn toggle_solution(&mut self) {
        self.show_solution = !self.show_solution;
    }

    pub fn toggle_files(&mut self) {
        self.show_files = !self.show_files;
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn reset_exercise(&mut self) {
        self.reset_exercise_state();
    }

    pub fn clear_output(&mut self) {
        self.last_output = None;
        self.submit_state = SubmitState::Idle;
        self.output_scroll = 0;
    }

    // --- Scrolling ---

    pub fn scroll_up(&mut self) {
        match self.current_view {
            ContentView::Intro => self.intro_scroll = self.intro_scroll.saturating_sub(1),
            ContentView::Examples => self.examples_scroll = self.examples_scroll.saturating_sub(1),
            ContentView::Exercise => self.output_scroll = self.output_scroll.saturating_sub(1),
        }
    }

    pub fn scroll_down(&mut self) {
        match self.current_view {
            ContentView::Intro => self.intro_scroll += 1,
            ContentView::Examples => self.examples_scroll += 1,
            ContentView::Exercise => self.output_scroll += 1,
        }
    }

    // --- Helpers ---

    fn reset_content_state(&mut self) {
        self.current_view = ContentView::Intro;
        self.current_exercise = 0;
        self.reset_exercise_state();
        self.intro_scroll = 0;
        self.examples_scroll = 0;
    }

    fn reset_exercise_state(&mut self) {
        self.clear_input();
        self.last_output = None;
        self.submit_state = SubmitState::Idle;
        self.hints_revealed = 0;
        self.show_solution = false;
        self.show_files = false;
        self.output_scroll = 0;
    }
}
