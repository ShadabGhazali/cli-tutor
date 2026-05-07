use cli_tutor::content::load_modules;

#[test]
fn all_bundled_toml_files_deserialize() {
    // command_modules.LOADING.2-3 — smoke test for all modules
    let modules = load_modules();
    assert!(!modules.is_empty(), "No modules loaded");
}

#[test]
fn twenty_five_modules_loaded_at_launch() {
    let modules = load_modules();
    assert_eq!(modules.len(), 25);
    let names: Vec<&str> = modules.iter().map(|m| m.module.name.as_str()).collect();
    // foundations
    assert!(names.contains(&"ls"));
    assert!(names.contains(&"cat"));
    assert!(names.contains(&"head"));
    assert!(names.contains(&"tail"));
    // search
    assert!(names.contains(&"grep"));
    assert!(names.contains(&"find"));
    // transform
    assert!(names.contains(&"cut"));
    assert!(names.contains(&"sort"));
    assert!(names.contains(&"uniq"));
    assert!(names.contains(&"wc"));
    assert!(names.contains(&"tr"));
    assert!(names.contains(&"sed"));
    assert!(names.contains(&"awk"));
    // combine/compare
    assert!(names.contains(&"paste"));
    assert!(names.contains(&"tee"));
    assert!(names.contains(&"diff"));
    // utilities
    assert!(names.contains(&"xargs"));
    assert!(names.contains(&"tar"));
    assert!(names.contains(&"chmod"));
    assert!(names.contains(&"bc"));
    // dev tools
    assert!(names.contains(&"git"));
    assert!(names.contains(&"jq"));
    assert!(names.contains(&"make"));
    // thematic workflows
    assert!(names.contains(&"log-processing"));
    assert!(names.contains(&"text-processing"));
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
