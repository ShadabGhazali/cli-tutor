use cli_tutor::content::load_modules;

#[test]
fn all_bundled_toml_files_deserialize() {
    // command_modules.LOADING.2-3 — smoke test for all modules
    let modules = load_modules();
    assert!(!modules.is_empty(), "No modules loaded");
}

#[test]
fn nine_modules_loaded_at_launch() {
    // command_modules.MODULES.1-9
    let modules = load_modules();
    assert_eq!(modules.len(), 9);
    let names: Vec<&str> = modules.iter().map(|m| m.module.name.as_str()).collect();
    assert!(names.contains(&"grep"));
    assert!(names.contains(&"awk"));
    assert!(names.contains(&"sed"));
    assert!(names.contains(&"find"));
    assert!(names.contains(&"xargs"));
    assert!(names.contains(&"cut"));
    assert!(names.contains(&"sort"));
    assert!(names.contains(&"uniq"));
    assert!(names.contains(&"tr"));
}

#[test]
fn all_exercise_ids_are_unique() {
    // command_modules.VALIDATION.1
    let modules = load_modules();
    let mut ids = std::collections::HashSet::new();
    for m in &modules {
        for ex in &m.exercises {
            assert!(ids.insert(ex.id.clone()), "Duplicate ID: {}", ex.id);
        }
    }
}

#[test]
fn all_modules_have_intro_and_exercises() {
    let modules = load_modules();
    for m in &modules {
        assert!(
            !m.intro.text.is_empty(),
            "{} has empty intro",
            m.module.name
        );
        assert!(
            !m.exercises.is_empty(),
            "{} has no exercises",
            m.module.name
        );
    }
}

#[test]
fn exercise_ids_follow_naming_convention() {
    // Each ID should be "module.N"
    let modules = load_modules();
    for m in &modules {
        for ex in &m.exercises {
            assert!(
                ex.id.starts_with(&m.module.name),
                "Exercise ID '{}' does not start with module name '{}'",
                ex.id,
                m.module.name
            );
        }
    }
}
