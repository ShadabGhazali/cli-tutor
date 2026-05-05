use cli_tutor::progress::Progress;
use std::sync::atomic::{AtomicU64, Ordering};

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

fn tmp_data_dir() -> std::path::PathBuf {
    let n = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("cli-tutor-itest-{}-{}", std::process::id(), n));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn save_progress(p: &Progress, data_dir: &std::path::Path) {
    let path = data_dir.join("cli-tutor").join("progress.json");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, serde_json::to_string_pretty(p).unwrap()).unwrap();
}

fn load_progress(data_dir: &std::path::Path) -> Progress {
    let path = data_dir.join("cli-tutor").join("progress.json");
    if !path.exists() {
        return Progress::default();
    }
    let content = std::fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

#[test]
fn fresh_start_on_missing_file() {
    let dir = tmp_data_dir();
    let p = load_progress(&dir);
    assert!(p.modules.is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn save_and_load_round_trip() {
    let dir = tmp_data_dir();
    let mut p = Progress::default();
    p.mark_completed("grep", "grep.1");
    p.mark_completed("grep", "grep.2");
    save_progress(&p, &dir);

    let loaded = load_progress(&dir);
    let grep = loaded.modules.get("grep").expect("grep module missing");
    assert!(grep.completed.contains(&"grep.1".to_string()));
    assert!(grep.completed.contains(&"grep.2".to_string()));
    assert!(!grep.completed.contains(&"grep.3".to_string()));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn fresh_start_on_corrupt_file() {
    let dir = tmp_data_dir();
    let path = dir.join("cli-tutor").join("progress.json");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "{{invalid json").unwrap();

    let result: Result<Progress, _> =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap());
    assert!(result.is_err());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn completed_not_double_counted() {
    let mut p = Progress::default();
    p.mark_completed("awk", "awk.1");
    p.mark_completed("awk", "awk.1");
    assert_eq!(p.modules["awk"].completed.len(), 1);
}
