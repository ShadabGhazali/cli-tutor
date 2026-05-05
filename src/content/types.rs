use serde::Deserialize;

// command_modules.ENG.1 — content structs derive Clone, Debug, Deserialize
#[derive(Debug, Clone, Deserialize)]
pub struct ModuleFile {
    pub module: ModuleInfo,
    pub intro: Intro,
    #[serde(default)]
    pub examples: Vec<Example>,
    #[serde(default)]
    pub exercises: Vec<Exercise>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModuleInfo {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub version: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Intro {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Example {
    pub title: String,
    pub command: String,
    pub output: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub difficulty: Difficulty,
    pub question: String,
    pub expected_output: String,
    #[serde(default)]
    pub hints: Vec<String>,
    pub solution: String,
    pub match_mode: MatchMode,
    // command_modules.SCHEMA.8 — fixtures optional
    #[serde(default)]
    pub fixtures: Vec<Fixture>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Fixture {
    pub filename: String,
    pub content: String,
}

// command_modules.SCHEMA.6
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Beginner => write!(f, "Beginner"),
            Difficulty::Intermediate => write!(f, "Intermediate"),
            Difficulty::Advanced => write!(f, "Advanced"),
        }
    }
}

// command_modules.SCHEMA.7
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MatchMode {
    Exact,
    Normalized,
    Sorted,
    Regex,
}
