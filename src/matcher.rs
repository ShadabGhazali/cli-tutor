use crate::content::types::MatchMode;

pub struct Matcher;

impl Matcher {
    /// Returns true if `actual` satisfies `expected` under the given `mode`.
    pub fn check(actual: &str, expected: &str, mode: &MatchMode) -> bool {
        match mode {
            // exact — byte-for-byte match
            MatchMode::Exact => actual == expected,

            // normalized — strip trailing whitespace per line, drop trailing blank lines
            MatchMode::Normalized => normalize(actual) == normalize(expected),

            // sorted — sort both sides' lines before comparing (normalized)
            MatchMode::Sorted => {
                let mut a = sorted_lines(actual);
                let mut e = sorted_lines(expected);
                a.sort_unstable();
                e.sort_unstable();
                a == e
            }

            // regex — expected is a pattern that must match actual
            MatchMode::Regex => regex_match(actual, expected),
        }
    }
}

fn normalize(s: &str) -> String {
    let mut lines: Vec<&str> = s.lines().map(|l| l.trim_end()).collect();
    while lines.last().map(|l| l.is_empty()).unwrap_or(false) {
        lines.pop();
    }
    lines.join("\n")
}

fn sorted_lines(s: &str) -> Vec<String> {
    s.lines()
        .map(|l| l.trim_end().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

fn regex_match(actual: &str, pattern: &str) -> bool {
    // Minimal regex support using stdlib — only anchored literal matching for now.
    // A proper regex crate can be added if needed without API changes.
    // For the v1.0 exercises, patterns are simple enough that contains() covers most cases.
    // This is an intentional simplification flagged here: regex.MATCH.1
    actual.contains(pattern.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_match_passes() {
        assert!(Matcher::check("hello\n", "hello\n", &MatchMode::Exact));
    }

    #[test]
    fn exact_match_fails_on_diff() {
        assert!(!Matcher::check("hello\n", "world\n", &MatchMode::Exact));
    }

    #[test]
    fn normalized_strips_trailing_space() {
        assert!(Matcher::check(
            "hello  \n",
            "hello\n",
            &MatchMode::Normalized
        ));
    }

    #[test]
    fn normalized_strips_trailing_blank_lines() {
        assert!(Matcher::check(
            "hello\n\n\n",
            "hello\n",
            &MatchMode::Normalized
        ));
    }

    #[test]
    fn sorted_order_independent() {
        assert!(Matcher::check("b\na\nc\n", "a\nb\nc\n", &MatchMode::Sorted));
    }

    #[test]
    fn sorted_fails_on_missing_line() {
        assert!(!Matcher::check("a\nb\n", "a\nb\nc\n", &MatchMode::Sorted));
    }

    #[test]
    fn regex_simple_match() {
        assert!(Matcher::check("hello world", "hello", &MatchMode::Regex));
    }

    #[test]
    fn regex_no_match() {
        assert!(!Matcher::check("hello world", "goodbye", &MatchMode::Regex));
    }
}
