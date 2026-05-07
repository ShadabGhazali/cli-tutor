use crate::config::Config;
use crate::content::types::{Difficulty, Exercise, ModuleFile};
use crate::executor::{ExecutionResult, Executor};
use crate::matcher::Matcher;
use crate::progress::{ModuleProgress, Progress, Stats};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentView {
    Intro,
    Examples,
    Exercise,
    // free_practice.VIEW.1 — free-practice is the 4th view in the cycle
    FreePractice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubmitState {
    Idle,
    Correct,
    Wrong,
    Error,
}

// difficulty_filter.FILTER.1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifficultyFilter {
    None,
    Beginner,
    Intermediate,
    Advanced,
}

impl std::fmt::Display for DifficultyFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DifficultyFilter::None => write!(f, "All"),
            DifficultyFilter::Beginner => write!(f, "Beginner"),
            DifficultyFilter::Intermediate => write!(f, "Intermediate"),
            DifficultyFilter::Advanced => write!(f, "Advanced"),
        }
    }
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
    // gamification.PERSIST.1 — stats live separately from per-module progress
    pub stats: Stats,
    pub should_quit: bool,

    // config_file.CONFIG.3
    pub config: Config,

    // command_history.HISTORY.1
    pub command_history: Vec<String>,
    pub history_idx: Option<usize>,
    history_draft: String,

    // module_search.SEARCH.1
    pub search_active: bool,
    pub search_query: String,
    pub search_filtered: Vec<usize>,

    // progress_summary.OVERLAY.1
    pub show_progress: bool,

    // difficulty_filter.FILTER.1
    pub difficulty_filter: DifficultyFilter,

    // timed_challenge.TIMER.1
    pub timer_start: Option<std::time::Instant>,
    pub last_solve_ms: Option<u64>,
}

impl App {
    pub fn new(modules: Vec<ModuleFile>, config: Config) -> Self {
        let progress = Progress::load();
        let stats = Stats::load();

        // config_file.CONFIG.2 — jump to default_module if configured
        let selected_module = config
            .default_module
            .as_deref()
            .and_then(|name| modules.iter().position(|m| m.module.name == name))
            .unwrap_or(0);

        App {
            modules,
            selected_module,
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
            stats,
            should_quit: false,
            config,
            command_history: Vec::new(),
            history_idx: None,
            history_draft: String::new(),
            search_active: false,
            search_query: String::new(),
            search_filtered: Vec::new(),
            show_progress: false,
            difficulty_filter: DifficultyFilter::None,
            timer_start: None,
            last_solve_ms: None,
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

    // module_search.SEARCH.3 — visible module indices (filtered or all)
    pub fn visible_module_indices(&self) -> Vec<usize> {
        if self.search_active && !self.search_query.is_empty() {
            self.search_filtered.clone()
        } else {
            (0..self.modules.len()).collect()
        }
    }

    // --- Input handling ---

    pub fn input_push(&mut self, c: char) {
        // timed_challenge.TIMER.1 — start timer on first keystroke in exercise
        if self.config.timed_challenge
            && self.current_view == ContentView::Exercise
            && self.timer_start.is_none()
        {
            self.timer_start = Some(std::time::Instant::now());
        }
        self.input.insert(self.cursor_pos, c);
        self.cursor_pos += c.len_utf8();
        // command_history.HISTORY.4 — typing cancels history navigation
        self.history_idx = None;
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
        self.history_idx = None;
    }

    pub fn input_delete(&mut self) {
        if self.cursor_pos < self.input.len() {
            self.input.remove(self.cursor_pos);
        }
        self.history_idx = None;
    }

    // paste_support.INPUT.2 — insert pasted text, skip control chars
    pub fn input_paste(&mut self, s: &str) {
        for c in s.chars() {
            if !c.is_control() {
                self.input.insert(self.cursor_pos, c);
                self.cursor_pos += c.len_utf8();
            }
        }
        self.history_idx = None;
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

    // word_jump.CURSOR.1 — jump to start of previous word
    pub fn cursor_word_left(&mut self) {
        let mut pos = self.cursor_pos;
        // skip non-word chars going left
        while pos > 0 {
            let c = self.input[..pos].chars().last().unwrap();
            if c.is_alphanumeric() || c == '_' {
                break;
            }
            pos -= c.len_utf8();
        }
        // skip word chars going left
        while pos > 0 {
            let c = self.input[..pos].chars().last().unwrap();
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            pos -= c.len_utf8();
        }
        self.cursor_pos = pos;
    }

    // word_jump.CURSOR.2 — jump past end of next word
    pub fn cursor_word_right(&mut self) {
        let mut pos = self.cursor_pos;
        let len = self.input.len();
        // skip non-word chars going right
        while pos < len {
            let c = self.input[pos..].chars().next().unwrap();
            if c.is_alphanumeric() || c == '_' {
                break;
            }
            pos += c.len_utf8();
        }
        // skip word chars going right
        while pos < len {
            let c = self.input[pos..].chars().next().unwrap();
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            pos += c.len_utf8();
        }
        self.cursor_pos = pos;
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
        self.history_idx = None;
    }

    // --- Command history ---

    // command_history.HISTORY.2 — push on submit, no consecutive duplicates
    fn push_history(&mut self, cmd: &str) {
        let cmd = cmd.trim().to_string();
        if cmd.is_empty() {
            return;
        }
        if self.command_history.last().map(|s| s == &cmd).unwrap_or(false) {
            return;
        }
        self.command_history.push(cmd);
        self.history_idx = None;
        self.history_draft.clear();
    }

    // command_history.HISTORY.3 — Up: load previous command
    pub fn history_prev(&mut self) {
        if self.command_history.is_empty() {
            return;
        }
        match self.history_idx {
            None => {
                self.history_draft = self.input.clone();
                self.history_idx = Some(self.command_history.len() - 1);
            }
            Some(0) => return,
            Some(i) => {
                self.history_idx = Some(i - 1);
            }
        }
        if let Some(i) = self.history_idx {
            self.input = self.command_history[i].clone();
            self.cursor_pos = self.input.len();
        }
    }

    // command_history.HISTORY.3 — Down: move toward present / restore draft
    pub fn history_next(&mut self) {
        match self.history_idx {
            None => {}
            Some(i) if i + 1 >= self.command_history.len() => {
                self.history_idx = None;
                self.input = self.history_draft.clone();
                self.cursor_pos = self.input.len();
            }
            Some(i) => {
                self.history_idx = Some(i + 1);
                self.input = self.command_history[i + 1].clone();
                self.cursor_pos = self.input.len();
            }
        }
    }

    // --- Module search ---

    // module_search.SEARCH.1 — activate search mode
    pub fn activate_search(&mut self) {
        self.search_active = true;
        self.search_query.clear();
        self.search_filtered = (0..self.modules.len()).collect();
    }

    // module_search.SEARCH.2 — append char and refilter
    pub fn search_push(&mut self, c: char) {
        self.search_query.push(c);
        self.update_search_filtered();
        if let Some(&first) = self.search_filtered.first() {
            self.selected_module = first;
        }
    }

    pub fn search_backspace(&mut self) {
        self.search_query.pop();
        self.update_search_filtered();
        if let Some(&first) = self.search_filtered.first() {
            self.selected_module = first;
        }
    }

    // module_search.SEARCH.4 — confirm selects first match
    pub fn search_confirm(&mut self) {
        if let Some(&first) = self.search_filtered.first() {
            self.selected_module = first;
            self.reset_content_state();
        }
        self.search_active = false;
        self.search_query.clear();
    }

    // module_search.SEARCH.5 — Esc cancels search
    pub fn search_cancel(&mut self) {
        self.search_active = false;
        self.search_query.clear();
        self.search_filtered.clear();
    }

    fn update_search_filtered(&mut self) {
        let q = self.search_query.to_lowercase();
        self.search_filtered = self
            .modules
            .iter()
            .enumerate()
            .filter(|(_, m)| m.module.name.to_lowercase().contains(&q))
            .map(|(i, _)| i)
            .collect();
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

    // free_practice.VIEW.1 — 4-step cycle including FreePractice
    pub fn cycle_view(&mut self) {
        self.current_view = match self.current_view {
            ContentView::Intro => ContentView::Examples,
            ContentView::Examples => ContentView::Exercise,
            ContentView::Exercise => ContentView::FreePractice,
            ContentView::FreePractice => ContentView::Intro,
        };
    }

    // difficulty_filter.FILTER.2 — cycle filter states
    pub fn cycle_difficulty_filter(&mut self) {
        self.difficulty_filter = match self.difficulty_filter {
            DifficultyFilter::None => DifficultyFilter::Beginner,
            DifficultyFilter::Beginner => DifficultyFilter::Intermediate,
            DifficultyFilter::Intermediate => DifficultyFilter::Advanced,
            DifficultyFilter::Advanced => DifficultyFilter::None,
        };
        // difficulty_filter.FILTER.4 — jump to first matching exercise on filter change
        if self.current_view == ContentView::Exercise {
            self.jump_to_first_matching_exercise();
        }
    }

    // difficulty_filter.FILTER.3 — skip non-matching in forward navigation
    pub fn next_exercise(&mut self) {
        let count = self.exercise_count();
        if count == 0 {
            return;
        }
        let mut candidate = self.current_exercise;
        loop {
            if candidate + 1 >= count {
                break;
            }
            candidate += 1;
            if self.exercise_matches_filter(candidate) {
                self.current_exercise = candidate;
                self.reset_exercise_state();
                return;
            }
        }
    }

    pub fn prev_exercise(&mut self) {
        if self.current_exercise == 0 {
            return;
        }
        let mut candidate = self.current_exercise;
        loop {
            if candidate == 0 {
                break;
            }
            candidate -= 1;
            if self.exercise_matches_filter(candidate) {
                self.current_exercise = candidate;
                self.reset_exercise_state();
                return;
            }
        }
    }

    fn exercise_matches_filter(&self, idx: usize) -> bool {
        let ex = match self.current_module().exercises.get(idx) {
            Some(e) => e,
            None => return false,
        };
        // config_file.CONFIG.7 — skip completed exercises when skip_completed is set
        if self.config.skip_completed {
            let module_name = &self.current_module().module.name;
            if self.progress.is_completed(module_name, &ex.id) {
                return false;
            }
        }
        match self.difficulty_filter {
            DifficultyFilter::None => true,
            DifficultyFilter::Beginner => ex.difficulty == Difficulty::Beginner,
            DifficultyFilter::Intermediate => ex.difficulty == Difficulty::Intermediate,
            DifficultyFilter::Advanced => ex.difficulty == Difficulty::Advanced,
        }
    }

    fn jump_to_first_matching_exercise(&mut self) {
        let count = self.exercise_count();
        for i in 0..count {
            if self.exercise_matches_filter(i) {
                self.current_exercise = i;
                return;
            }
        }
    }

    // --- Exercise actions ---

    pub fn submit_command(&mut self) {
        let command = self.input.clone();
        if command.trim().is_empty() {
            return;
        }

        // command_history.HISTORY.2
        self.push_history(&command);

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
            // gamification.XP.2 — only award XP on the first correct solve
            let already_completed = self.progress.is_completed(&module_name, &exercise.id);
            self.progress.mark_completed(&module_name, &exercise.id);

            // timed_challenge.TIMER.2 — stop timer and record best time
            if self.config.timed_challenge {
                if let Some(start) = self.timer_start.take() {
                    let ms = start.elapsed().as_millis() as u64;
                    self.last_solve_ms = Some(ms);
                    self.progress.record_time(&module_name, &exercise.id, ms);
                }
            }

            // gamification.XP.1 — award XP based on difficulty, first solve only
            if !already_completed {
                let xp: u64 = match exercise.difficulty {
                    Difficulty::Beginner => 10,
                    Difficulty::Intermediate => 20,
                    Difficulty::Advanced => 30,
                };
                self.stats.add_xp(xp);
                self.stats.update_streak();
                let _ = self.stats.save();
            }

            let _ = self.progress.save();
        } else {
            self.submit_state = SubmitState::Wrong;
            self.progress.mark_attempted(&module_name, &exercise.id);
        }
    }

    // free_practice.VIEW.2 — run command with no expected-output matching
    pub fn submit_command_free(&mut self) {
        let command = self.input.clone();
        if command.trim().is_empty() {
            return;
        }
        self.push_history(&command);

        let result = Executor::run(&command, &[]);
        self.last_output = match result {
            Ok(o) => Some(o),
            Err(e) => Some(ExecutionResult {
                stdout: String::new(),
                stderr: e.to_string(),
                timed_out: false,
            }),
        };
        self.submit_state = SubmitState::Idle;
        self.output_scroll = 0;
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

    // progress_summary.OVERLAY.4 — toggle progress overlay
    pub fn toggle_progress(&mut self) {
        self.show_progress = !self.show_progress;
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
            ContentView::Examples => {
                self.examples_scroll = self.examples_scroll.saturating_sub(1)
            }
            ContentView::Exercise | ContentView::FreePractice => {
                self.output_scroll = self.output_scroll.saturating_sub(1)
            }
        }
    }

    pub fn scroll_down(&mut self) {
        match self.current_view {
            ContentView::Intro => self.intro_scroll += 1,
            ContentView::Examples => self.examples_scroll += 1,
            ContentView::Exercise | ContentView::FreePractice => self.output_scroll += 1,
        }
    }

    // --- Helpers ---

    fn reset_content_state(&mut self) {
        self.current_view = ContentView::Intro;
        self.current_exercise = 0;
        self.reset_exercise_state();
        self.intro_scroll = 0;
        self.examples_scroll = 0;
        // difficulty_filter.FILTER.4 — reset filter on module switch
        self.difficulty_filter = DifficultyFilter::None;
    }

    fn reset_exercise_state(&mut self) {
        self.clear_input();
        self.last_output = None;
        self.submit_state = SubmitState::Idle;
        self.hints_revealed = 0;
        self.show_solution = false;
        self.show_files = false;
        self.output_scroll = 0;
        // timed_challenge.TIMER.1 — reset timer between exercises
        self.timer_start = None;
        self.last_solve_ms = None;
    }
}
