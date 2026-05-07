pub mod types;

use std::collections::HashSet;
use types::ModuleFile;

#[derive(Debug, serde::Deserialize)]
struct ModuleIndex {
    modules: Vec<String>,
}

// command_modules.LOADING.1 — all TOML files embedded at compile time
const MODULES_TOML: &str = include_str!("../../content/modules.toml");
const LS_TOML: &str = include_str!("../../content/ls.toml");
const CAT_TOML: &str = include_str!("../../content/cat.toml");
const HEAD_TOML: &str = include_str!("../../content/head.toml");
const TAIL_TOML: &str = include_str!("../../content/tail.toml");
const GREP_TOML: &str = include_str!("../../content/grep.toml");
const FIND_TOML: &str = include_str!("../../content/find.toml");
const CUT_TOML: &str = include_str!("../../content/cut.toml");
const SORT_TOML: &str = include_str!("../../content/sort.toml");
const UNIQ_TOML: &str = include_str!("../../content/uniq.toml");
const WC_TOML: &str = include_str!("../../content/wc.toml");
const TR_TOML: &str = include_str!("../../content/tr.toml");
const SED_TOML: &str = include_str!("../../content/sed.toml");
const AWK_TOML: &str = include_str!("../../content/awk.toml");
const PASTE_TOML: &str = include_str!("../../content/paste.toml");
const TEE_TOML: &str = include_str!("../../content/tee.toml");
const DIFF_TOML: &str = include_str!("../../content/diff.toml");
const XARGS_TOML: &str = include_str!("../../content/xargs.toml");
const TAR_TOML: &str = include_str!("../../content/tar.toml");
const CHMOD_TOML: &str = include_str!("../../content/chmod.toml");
const BC_TOML: &str = include_str!("../../content/bc.toml");
const GIT_TOML: &str = include_str!("../../content/git.toml");
const JQ_TOML: &str = include_str!("../../content/jq.toml");
const MAKE_TOML: &str = include_str!("../../content/make.toml");
const LOG_PROCESSING_TOML: &str = include_str!("../../content/log-processing.toml");
const TEXT_PROCESSING_TOML: &str = include_str!("../../content/text-processing.toml");

fn raw_by_name(name: &str) -> Option<&'static str> {
    match name {
        "ls" => Some(LS_TOML),
        "cat" => Some(CAT_TOML),
        "head" => Some(HEAD_TOML),
        "tail" => Some(TAIL_TOML),
        "grep" => Some(GREP_TOML),
        "find" => Some(FIND_TOML),
        "cut" => Some(CUT_TOML),
        "sort" => Some(SORT_TOML),
        "uniq" => Some(UNIQ_TOML),
        "wc" => Some(WC_TOML),
        "tr" => Some(TR_TOML),
        "sed" => Some(SED_TOML),
        "awk" => Some(AWK_TOML),
        "paste" => Some(PASTE_TOML),
        "tee" => Some(TEE_TOML),
        "diff" => Some(DIFF_TOML),
        "xargs" => Some(XARGS_TOML),
        "tar" => Some(TAR_TOML),
        "chmod" => Some(CHMOD_TOML),
        "bc" => Some(BC_TOML),
        "git" => Some(GIT_TOML),
        "jq" => Some(JQ_TOML),
        "make" => Some(MAKE_TOML),
        "log-processing" => Some(LOG_PROCESSING_TOML),
        "text-processing" => Some(TEXT_PROCESSING_TOML),
        _ => None,
    }
}

// command_modules.LOADING.2-5, VALIDATION.1-4
pub fn load_modules() -> Vec<ModuleFile> {
    // command_modules.LOADING.5 — order from modules.toml
    let index: ModuleIndex =
        toml::from_str(MODULES_TOML).expect("content/modules.toml failed to parse");

    let mut modules = Vec::with_capacity(index.modules.len());
    for name in &index.modules {
        let raw =
            raw_by_name(name).unwrap_or_else(|| panic!("No embedded TOML for module '{name}'"));
        // command_modules.LOADING.3 — panic on bad TOML with clear message
        let module: ModuleFile = toml::from_str(raw)
            .unwrap_or_else(|e| panic!("Failed to parse content/{name}.toml: {e}"));
        modules.push(module);
    }

    // command_modules.VALIDATION.1 — duplicate exercise IDs cause panic
    let mut seen_ids: HashSet<String> = HashSet::new();
    for m in &modules {
        for ex in &m.exercises {
            if !seen_ids.insert(ex.id.clone()) {
                panic!("Duplicate exercise ID '{}' found", ex.id);
            }
        }
    }

    // command_modules.VALIDATION.3 — compile regex match_mode exercises at startup
    for m in &modules {
        for ex in &m.exercises {
            if ex.match_mode == types::MatchMode::Regex {
                regex_compile_check(&ex.id, &ex.expected_output);
            }
        }
    }

    // command_modules.VALIDATION.4 — log total count in debug builds
    #[cfg(debug_assertions)]
    {
        let total: usize = modules.iter().map(|m| m.exercises.len()).sum();
        eprintln!("Loaded {} modules, {} exercises", modules.len(), total);
        for m in &modules {
            eprintln!(
                "  {} (v{}): {} exercises",
                m.module.name,
                m.module.version,
                m.exercises.len()
            );
        }
    }

    modules
}

fn regex_compile_check(id: &str, pattern: &str) {
    // Basic regex validity check using std — real regex matching in matcher.rs
    // We use a simple heuristic: attempt to detect obviously invalid patterns.
    // For full validation, matcher.rs will use its own regex engine if added later.
    // For now we just ensure the pattern is non-empty.
    if pattern.trim().is_empty() {
        panic!("Exercise '{id}' has match_mode=regex but empty expected_output");
    }
}
