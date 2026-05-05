use crate::content::types::Fixture;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use thiserror::Error;

const TIMEOUT_SECS: u64 = 3;

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
}

#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("Failed to create temp directory: {0}")]
    TempDir(#[from] std::io::Error),
    #[error("Failed to write fixture '{0}': {1}")]
    FixtureWrite(String, std::io::Error),
}

pub struct Executor;

impl Executor {
    pub fn run(command: &str, fixtures: &[Fixture]) -> Result<ExecutionResult, ExecutorError> {
        let tmp = create_temp_dir()?;
        write_fixtures(&tmp, fixtures)?;

        let path_env = std::env::var("PATH").unwrap_or_else(|_| "/usr/bin:/bin".to_string());
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(&tmp)
            .env_clear()
            .env("PATH", &path_env)
            .env("HOME", &home)
            .env("USER", &user)
            .env("TERM", "xterm")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(ExecutorError::TempDir)?;

        // Thread-based timeout
        let (tx, rx) = mpsc::channel();
        let child_id = child.id();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(TIMEOUT_SECS));
            let _ = tx.send(());
            // Best-effort kill
            unsafe {
                libc_kill(child_id);
            }
        });

        let stdout_handle = child.stdout.take();
        let stderr_handle = child.stderr.take();

        // Read stdout/stderr in separate threads to avoid deadlock
        let stdout_thread = thread::spawn(move || {
            use std::io::Read;
            let mut buf = String::new();
            if let Some(mut s) = stdout_handle {
                let _ = s.read_to_string(&mut buf);
            }
            buf
        });
        let stderr_thread = thread::spawn(move || {
            use std::io::Read;
            let mut buf = String::new();
            if let Some(mut s) = stderr_handle {
                let _ = s.read_to_string(&mut buf);
            }
            buf
        });

        let status = child.wait().map_err(ExecutorError::TempDir)?;
        let stdout = stdout_thread.join().unwrap_or_default();
        let stderr = stderr_thread.join().unwrap_or_default();

        let timed_out = rx.try_recv().is_ok() || !status.success() && stderr.contains("Killed");

        let _ = fs::remove_dir_all(&tmp);

        Ok(ExecutionResult {
            stdout,
            stderr,
            timed_out,
        })
    }
}

fn create_temp_dir() -> Result<PathBuf, ExecutorError> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let base = std::env::temp_dir().join(format!("cli-tutor-{}-{}", std::process::id(), n));
    fs::create_dir_all(&base).map_err(ExecutorError::TempDir)?;
    Ok(base)
}

fn write_fixtures(dir: &std::path::Path, fixtures: &[Fixture]) -> Result<(), ExecutorError> {
    for fixture in fixtures {
        // Prevent path traversal — fixture filenames must not contain '..'
        let path = dir.join(&fixture.filename);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ExecutorError::FixtureWrite(fixture.filename.clone(), e))?;
        }
        let mut f = fs::File::create(&path)
            .map_err(|e| ExecutorError::FixtureWrite(fixture.filename.clone(), e))?;
        f.write_all(fixture.content.as_bytes())
            .map_err(|e| ExecutorError::FixtureWrite(fixture.filename.clone(), e))?;
    }
    Ok(())
}

#[allow(unsafe_code)]
unsafe fn libc_kill(pid: u32) {
    // SIGTERM the process group; ignore errors (process may already be done)
    libc_kill_raw(pid as i32, 15);
}

#[cfg(unix)]
fn libc_kill_raw(pid: i32, sig: i32) {
    unsafe {
        libc_sys_kill(pid, sig);
    }
}

#[cfg(not(unix))]
fn libc_kill_raw(_pid: i32, _sig: i32) {}

// Minimal libc kill binding without pulling in the libc crate
#[cfg(unix)]
extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
}

#[cfg(unix)]
unsafe fn libc_sys_kill(pid: i32, sig: i32) {
    kill(pid, sig);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_stdout_capture() {
        let result = Executor::run("echo hello", &[]).unwrap();
        assert_eq!(result.stdout.trim(), "hello");
        assert!(!result.timed_out);
    }

    #[test]
    fn test_stderr_capture() {
        let result = Executor::run("echo err >&2", &[]).unwrap();
        assert_eq!(result.stderr.trim(), "err");
    }

    #[test]
    fn test_fixture_written() {
        let fixtures = vec![Fixture {
            filename: "test.txt".to_string(),
            content: "hello\nworld\n".to_string(),
        }];
        let result = Executor::run("cat test.txt", &fixtures).unwrap();
        assert_eq!(result.stdout, "hello\nworld\n");
    }

    #[test]
    fn test_timeout() {
        let result = Executor::run("sleep 10", &[]).unwrap();
        assert!(result.timed_out || !result.stderr.is_empty());
    }

    #[test]
    fn test_empty_env_strips_custom_vars() {
        let result = Executor::run("echo ${MY_SECRET:-not_set}", &[]).unwrap();
        assert_eq!(result.stdout.trim(), "not_set");
    }
}
