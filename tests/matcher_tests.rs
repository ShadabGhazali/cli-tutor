use cli_tutor::content::types::MatchMode;
use cli_tutor::matcher::Matcher;

#[test]
fn exact_same_string_passes() {
    assert!(Matcher::check("foo\n", "foo\n", &MatchMode::Exact));
}

#[test]
fn exact_different_string_fails() {
    assert!(!Matcher::check("foo\n", "bar\n", &MatchMode::Exact));
}

#[test]
fn exact_trailing_space_fails() {
    assert!(!Matcher::check("foo  \n", "foo\n", &MatchMode::Exact));
}

#[test]
fn normalized_trailing_space_passes() {
    assert!(Matcher::check("foo  \n", "foo\n", &MatchMode::Normalized));
}

#[test]
fn normalized_trailing_blank_lines_passes() {
    assert!(Matcher::check("foo\n\n\n", "foo\n", &MatchMode::Normalized));
}

#[test]
fn normalized_different_content_fails() {
    assert!(!Matcher::check("foo\n", "bar\n", &MatchMode::Normalized));
}

#[test]
fn sorted_different_order_passes() {
    assert!(Matcher::check("b\na\nc\n", "a\nb\nc\n", &MatchMode::Sorted));
}

#[test]
fn sorted_missing_line_fails() {
    assert!(!Matcher::check("a\nb\n", "a\nb\nc\n", &MatchMode::Sorted));
}

#[test]
fn sorted_extra_line_fails() {
    assert!(!Matcher::check("a\nb\nc\n", "a\nb\n", &MatchMode::Sorted));
}

#[test]
fn regex_substring_passes() {
    assert!(Matcher::check("hello world\n", "hello", &MatchMode::Regex));
}

#[test]
fn regex_no_match_fails() {
    assert!(!Matcher::check(
        "hello world\n",
        "goodbye",
        &MatchMode::Regex
    ));
}
