use cli_tutor::content::types::Fixture;
use cli_tutor::executor::Executor;

#[test]
fn stdout_is_captured() {
    let r = Executor::run("echo hello", &[]).unwrap();
    assert_eq!(r.stdout.trim(), "hello");
}

#[test]
fn stderr_is_captured_separately() {
    let r = Executor::run("echo error >&2", &[]).unwrap();
    assert!(r.stdout.is_empty() || r.stdout == "\n");
    assert!(r.stderr.contains("error"));
}

#[test]
fn fixture_is_written_and_readable() {
    let fixtures = vec![Fixture {
        filename: "greet.txt".to_string(),
        content: "hello world\n".to_string(),
    }];
    let r = Executor::run("cat greet.txt", &fixtures).unwrap();
    assert_eq!(r.stdout, "hello world\n");
}

#[test]
fn nested_fixture_directory_is_created() {
    let fixtures = vec![Fixture {
        filename: "subdir/file.txt".to_string(),
        content: "nested\n".to_string(),
    }];
    let r = Executor::run("cat subdir/file.txt", &fixtures).unwrap();
    assert_eq!(r.stdout, "nested\n");
}

#[test]
fn timeout_kills_long_running_command() {
    let r = Executor::run("sleep 10", &[]).unwrap();
    assert!(r.timed_out || !r.stderr.is_empty());
}

#[test]
fn env_is_stripped_of_custom_vars() {
    let r = Executor::run("echo ${MY_SECRET_VAR:-unset}", &[]).unwrap();
    assert_eq!(r.stdout.trim(), "unset");
}

#[test]
fn path_and_home_are_preserved() {
    let r = Executor::run("echo $HOME", &[]).unwrap();
    assert!(!r.stdout.trim().is_empty());

    let r2 = Executor::run("which sh", &[]).unwrap();
    assert!(r2.stdout.contains("sh"));
}
