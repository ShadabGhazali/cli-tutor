/// Runs every exercise's solution command against its fixtures and verifies
/// the output matches expected_output using the exercise's match_mode.
/// This is the most critical test — it catches broken TOML content.
use cli_tutor::content::load_modules;
use cli_tutor::executor::Executor;
use cli_tutor::matcher::Matcher;

#[test]
fn all_exercise_solutions_produce_correct_output() {
    let modules = load_modules();
    let mut failures: Vec<String> = Vec::new();

    for module in &modules {
        for exercise in &module.exercises {
            let result = match Executor::run(&exercise.solution, &exercise.fixtures) {
                Ok(r) => r,
                Err(e) => {
                    failures.push(format!(
                        "[{}] executor error: {}",
                        exercise.id, e
                    ));
                    continue;
                }
            };

            if result.timed_out {
                failures.push(format!("[{}] solution timed out", exercise.id));
                continue;
            }

            let pass = Matcher::check(
                &result.stdout,
                &exercise.expected_output,
                &exercise.match_mode,
            );

            if !pass {
                failures.push(format!(
                    "[{}] output mismatch\n  mode:     {:?}\n  expected: {:?}\n  got:      {:?}\n  stderr:   {:?}",
                    exercise.id,
                    exercise.match_mode,
                    exercise.expected_output,
                    result.stdout,
                    result.stderr,
                ));
            }
        }
    }

    if !failures.is_empty() {
        panic!(
            "\n{} exercise(s) failed:\n\n{}",
            failures.len(),
            failures.join("\n\n")
        );
    }
}

/// Each exercise with fixtures must have at least one fixture file.
#[test]
fn exercises_referencing_files_have_fixtures() {
    let modules = load_modules();
    for module in &modules {
        for exercise in &module.exercises {
            let q = &exercise.question;
            let mentions_file = q.contains(".txt")
                || q.contains(".csv")
                || q.contains(".log")
                || q.contains(".sh")
                || q.contains(".py")
                || q.contains(".ini")
                || q.contains("file");
            if mentions_file && exercise.fixtures.is_empty() {
                // Not a hard failure — some questions describe files in the question
                // without needing fixtures (e.g. /etc/passwd). Just a warning.
                eprintln!(
                    "Warning: [{}] mentions files but has no fixtures",
                    exercise.id
                );
            }
        }
    }
}

/// Fixture filenames must not start with '/' or contain '..'.
#[test]
fn fixture_filenames_are_safe() {
    let modules = load_modules();
    for module in &modules {
        for exercise in &module.exercises {
            for fixture in &exercise.fixtures {
                assert!(
                    !fixture.filename.starts_with('/'),
                    "[{}] fixture '{}' is an absolute path",
                    exercise.id,
                    fixture.filename
                );
                assert!(
                    !fixture.filename.contains(".."),
                    "[{}] fixture '{}' contains '..'",
                    exercise.id,
                    fixture.filename
                );
            }
        }
    }
}

/// Solutions must be non-empty strings.
#[test]
fn all_solutions_are_non_empty() {
    let modules = load_modules();
    for module in &modules {
        for exercise in &module.exercises {
            assert!(
                !exercise.solution.trim().is_empty(),
                "[{}] has an empty solution",
                exercise.id
            );
        }
    }
}

/// Expected outputs must be non-empty.
#[test]
fn all_expected_outputs_are_non_empty() {
    let modules = load_modules();
    for module in &modules {
        for exercise in &module.exercises {
            assert!(
                !exercise.expected_output.trim().is_empty(),
                "[{}] has empty expected_output",
                exercise.id
            );
        }
    }
}
