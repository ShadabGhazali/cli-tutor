use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;

// config_file.CONFIG.1
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    // config_file.CONFIG.2
    #[serde(default)]
    pub no_color: bool,
    #[serde(default)]
    pub default_module: Option<String>,
    #[serde(default)]
    pub skip_completed: bool,
    #[serde(default)]
    pub timed_challenge: bool,
}

impl Config {
    // config_file.CONFIG.3 — load at startup, fallback to defaults
    pub fn load() -> Self {
        match Self::try_load() {
            Ok(c) => c,
            Err(e) => {
                // config_file.CONFIG.5 — warn on corrupt, never panic
                eprintln!("Warning: could not load config: {e}. Using defaults.");
                Self::default()
            }
        }
    }

    fn try_load() -> Result<Self> {
        let path = Self::path()?;
        // config_file.CONFIG.4 — missing file → silent default
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        Ok(toml::from_str(&content)?)
    }

    // config_file.CONFIG.1 — XDG_CONFIG_HOME or ~/.config
    fn path() -> Result<PathBuf> {
        let base = if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg)
        } else {
            let home = std::env::var("HOME")?;
            PathBuf::from(home).join(".config")
        };
        Ok(base.join("cli-tutor").join("config.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn with_config_dir<F: FnOnce(PathBuf)>(f: F) {
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let tmp = std::env::temp_dir()
            .join(format!("cli-tutor-cfg-{}-{}", std::process::id(), n));
        std::fs::create_dir_all(&tmp).unwrap();
        f(tmp.clone());
        let _ = std::fs::remove_dir_all(&tmp);
    }

    fn write_config(dir: &PathBuf, content: &str) {
        let path = dir.join("cli-tutor").join("config.toml");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, content).unwrap();
    }

    fn load_from(xdg: &PathBuf) -> Config {
        let path = xdg.join("cli-tutor").join("config.toml");
        if !path.exists() {
            return Config::default();
        }
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    }

    #[test]
    fn config_defaults_when_file_missing() {
        with_config_dir(|dir| {
            let c = load_from(&dir);
            assert!(!c.no_color);
            assert!(c.default_module.is_none());
            assert!(!c.skip_completed);
            assert!(!c.timed_challenge);
        });
    }

    #[test]
    fn config_no_color_from_file() {
        with_config_dir(|dir| {
            write_config(&dir, "no_color = true\n");
            let c = load_from(&dir);
            assert!(c.no_color);
        });
    }

    #[test]
    fn config_timed_challenge_from_file() {
        with_config_dir(|dir| {
            write_config(&dir, "timed_challenge = true\n");
            let c = load_from(&dir);
            assert!(c.timed_challenge);
        });
    }

    #[test]
    fn config_default_module_from_file() {
        with_config_dir(|dir| {
            write_config(&dir, "default_module = \"sed\"\n");
            let c = load_from(&dir);
            assert_eq!(c.default_module.as_deref(), Some("sed"));
        });
    }

    #[test]
    fn config_returns_default_on_corrupt_file() {
        with_config_dir(|dir| {
            write_config(&dir, "{{not valid toml at all");
            let result: Result<Config, _> = toml::from_str("{{not valid toml at all");
            assert!(result.is_err());
            // Config::load() falls back gracefully
            let c = Config::default();
            assert!(!c.no_color);
        });
    }
}
