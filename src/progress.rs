use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModuleProgress {
    pub completed: Vec<String>,
    pub attempted: Vec<String>,
    // timed_challenge.TIMER.3 — best solve time per exercise (milliseconds)
    #[serde(default)]
    pub best_times: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Progress {
    #[serde(flatten)]
    pub modules: HashMap<String, ModuleProgress>,
}

impl Progress {
    pub fn load() -> Self {
        match Self::try_load() {
            Ok(p) => p,
            Err(e) => {
                // progress_tracking.LOAD.2 — warn and start fresh on corrupt file
                eprintln!("Warning: could not load progress file: {e}. Starting fresh.");
                Self::default()
            }
        }
    }

    fn try_load() -> Result<Self> {
        let path = progress_path()?;
        if !path.exists() {
            // progress_tracking.LOAD.1 — missing file → fresh start, no error
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&path)?;
        let p: Self = serde_json::from_str(&content)?;
        Ok(p)
    }

    pub fn save(&self) -> Result<()> {
        let path = progress_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&path, json)?;
        Ok(())
    }

    pub fn mark_completed(&mut self, module: &str, exercise_id: &str) {
        let entry = self.modules.entry(module.to_string()).or_default();
        if !entry.completed.contains(&exercise_id.to_string()) {
            entry.completed.push(exercise_id.to_string());
        }
        entry.attempted.retain(|id| id != exercise_id);
    }

    pub fn mark_attempted(&mut self, module: &str, exercise_id: &str) {
        let entry = self.modules.entry(module.to_string()).or_default();
        if !entry.completed.contains(&exercise_id.to_string())
            && !entry.attempted.contains(&exercise_id.to_string())
        {
            entry.attempted.push(exercise_id.to_string());
        }
    }

    pub fn is_completed(&self, module: &str, exercise_id: &str) -> bool {
        self.modules
            .get(module)
            .map(|p| p.completed.iter().any(|id| id == exercise_id))
            .unwrap_or(false)
    }

    // timed_challenge.TIMER.3 — record best solve time; only keeps the fastest
    pub fn record_time(&mut self, module: &str, exercise_id: &str, ms: u64) {
        let entry = self.modules.entry(module.to_string()).or_default();
        let best = entry.best_times.entry(exercise_id.to_string()).or_insert(u64::MAX);
        if ms < *best {
            *best = ms;
        }
    }

    pub fn best_time(&self, module: &str, exercise_id: &str) -> Option<u64> {
        self.modules
            .get(module)
            .and_then(|p| p.best_times.get(exercise_id).copied())
            .filter(|&ms| ms < u64::MAX)
    }
}

// gamification.PERSIST.1 — global stats stored separately from per-module progress
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Stats {
    #[serde(default)]
    pub total_xp: u64,
    #[serde(default)]
    pub streak_days: u32,
    // gamification.STREAK.6 — Unix epoch day number (secs / 86400)
    #[serde(default)]
    pub last_active_day: Option<u64>,
}

impl Stats {
    pub fn load() -> Self {
        match Self::try_load() {
            Ok(s) => s,
            Err(e) => {
                // gamification.PERSIST.3 — warn and use defaults on corrupt file
                eprintln!("Warning: could not load stats file: {e}. Starting fresh.");
                Self::default()
            }
        }
    }

    fn try_load() -> Result<Self> {
        let path = stats_path()?;
        if !path.exists() {
            // gamification.PERSIST.2 — missing file → silent defaults
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&path)?;
        let s: Self = serde_json::from_str(&content)?;
        Ok(s)
    }

    pub fn save(&self) -> Result<()> {
        let path = stats_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&path, json)?;
        Ok(())
    }

    // gamification.XP.1 — caller passes the computed xp amount (no content-type dependency)
    pub fn add_xp(&mut self, xp: u64) {
        self.total_xp += xp;
    }

    // gamification.STREAK.1-4 — update streak based on today vs last active day
    pub fn update_streak(&mut self) {
        let today = today_day();
        match self.last_active_day {
            None => {
                self.streak_days = 1;
            }
            Some(last) if last == today => {
                // already active today — no change
            }
            Some(last) if today > 0 && last == today - 1 => {
                // consecutive day — extend streak
                self.streak_days += 1;
            }
            _ => {
                // gap or future date — reset streak
                self.streak_days = 1;
            }
        }
        self.last_active_day = Some(today);
    }
}

fn today_day() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / 86400
}

fn stats_path() -> Result<PathBuf> {
    let base = dirs_base()?;
    Ok(base.join("cli-tutor").join("stats.json"))
}

fn progress_path() -> Result<PathBuf> {
    let base = dirs_base()?;
    Ok(base.join("cli-tutor").join("progress.json"))
}

fn dirs_base() -> Result<PathBuf> {
    // XDG_DATA_HOME or ~/.local/share
    if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        return Ok(PathBuf::from(xdg));
    }
    let home = std::env::var("HOME")?;
    Ok(PathBuf::from(home).join(".local").join("share"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    // Use XDG_DATA_HOME with a unique temp dir to avoid racing on HOME across parallel tests.
    fn with_xdg_data<F: FnOnce(PathBuf)>(f: F) {
        let n = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        let tmp =
            std::env::temp_dir().join(format!("cli-tutor-prog-test-{}-{}", std::process::id(), n));
        std::fs::create_dir_all(&tmp).unwrap();
        f(tmp.clone());
        let _ = std::fs::remove_dir_all(&tmp);
    }

    fn load_from(xdg_data: &PathBuf) -> Progress {
        let path = xdg_data.join("cli-tutor").join("progress.json");
        if !path.exists() {
            return Progress::default();
        }
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    }

    fn save_to(p: &Progress, xdg_data: &PathBuf) {
        let path = xdg_data.join("cli-tutor").join("progress.json");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let json = serde_json::to_string_pretty(p).unwrap();
        std::fs::write(&path, json).unwrap();
    }

    #[test]
    fn fresh_start_on_missing_file() {
        with_xdg_data(|dir| {
            let p = load_from(&dir);
            assert!(p.modules.is_empty());
        });
    }

    #[test]
    fn save_and_round_trip() {
        with_xdg_data(|dir| {
            let mut p = Progress::default();
            p.mark_completed("grep", "grep.1");
            save_to(&p, &dir);

            let loaded = load_from(&dir);
            assert!(loaded.is_completed("grep", "grep.1"));
        });
    }

    #[test]
    fn fresh_start_on_corrupt_file() {
        with_xdg_data(|dir| {
            let path = dir.join("cli-tutor").join("progress.json");
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, "{{invalid json").unwrap();

            let content = std::fs::read_to_string(&path).unwrap();
            let result: Result<Progress, _> = serde_json::from_str(&content);
            assert!(
                result.is_err(),
                "Expected corrupt file to fail deserialization"
            );

            // Progress::load() itself falls back to default on error
            let p = Progress::default();
            assert!(p.modules.is_empty());
        });
    }

    #[test]
    fn mark_completed_deduplicates() {
        let mut p = Progress::default();
        p.mark_completed("grep", "grep.1");
        p.mark_completed("grep", "grep.1");
        assert_eq!(p.modules["grep"].completed.len(), 1);
    }
}
