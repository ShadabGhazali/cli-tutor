# CLI Tutor — Claude Code Instructions

## Project Summary

CLI Tutor is a Rust TUI application for learning Unix command-line tools interactively.
Stack: **Rust + Ratatui + Crossterm**. Single binary, no runtime deps, event-driven (no tick loops).

Read `PRD.md` for the full product spec before starting any task.

---

## Spec-Driven Development

This project uses **spec-driven development**. Every feature has a `feature.yaml` file in `features/`.

### Rules — follow these on every task

1. **Before implementing any feature**, read the relevant `feature.yaml` in `features/`.
2. Each requirement has an **ACID** (Acceptance Criteria ID) in the format:
   `{feature-name}.{COMPONENT}.{number}` — e.g. `text_input.INPUT.3`
3. **Annotate code and tests** with ACID comments directly above the relevant line/block:
   ```rust
   // text_input.INPUT.3 — clears field on Enter submit
   self.input.clear();
   ```
4. Every requirement must have at least one corresponding test referencing its ACID.
5. **Do not modify feature.yaml files** unless explicitly asked. They are the source of truth.
6. If you believe a requirement is missing or wrong, flag it as a comment — do not silently deviate.

---

## Architecture Rules

### State & Rendering
- All application state lives in `src/app.rs` → `App` struct.
- Rendering is **event-driven only**. Do NOT add `tick`-based or polling loops for UI refresh.
- Ratatui's `terminal.draw()` is called only in response to a terminal event (key, resize).
- CPU usage at idle must be near zero.

### Module Boundaries
| File/Dir | Responsibility |
|---|---|
| `src/main.rs` | Entry point, terminal setup/teardown, top-level event loop |
| `src/app.rs` | App state machine, event dispatch |
| `src/ui/` | Pure rendering functions — take `&App`, return nothing, only call `frame.render_*` |
| `src/executor.rs` | Subprocess sandboxing, command execution, output capture |
| `src/matcher.rs` | Output comparison logic (exact, normalized, sorted, regex) |
| `src/progress.rs` | Load/save `~/.local/share/cli-tutor/progress.json` |
| `src/content/` | Content loader, module registry, deserialization of TOML files |
| `content/*.toml` | Command module data — intro text, examples, exercises, fixtures |

### UI rendering functions must be pure
UI functions in `src/ui/` must **never** mutate `App` state. They take `&App` (or specific slices) and a `Frame`, and only render. All mutations happen in `src/app.rs` in response to events.

### No unwrap() in production paths
Use `?`, `if let`, or explicit error handling. `unwrap()` is only acceptable in tests.

### Command Execution
- Always execute via `sh -c "<user_command>"` with working directory set to the fixture temp dir.
- Capture stdout and stderr separately using `std::process::Command`.
- Apply a 3-second timeout (use `std::process::Child::wait_timeout` or a thread + channel pattern).
- Never pass user input directly to a shell without going through `sh -c` (already handles quoting).
- Minimal environment: preserve `PATH`; set `HOME`, `USER`, `TERM`; strip everything else.

### Content Loading
- All TOML content files are embedded at compile time using `include_str!()` or `rust-embed`.
- The content registry is built once at startup and is immutable for the session.
- Content structs live in `src/content/types.rs`. Keep them `Clone` + `Debug` + `serde::Deserialize`.

### Progress
- Progress is persisted to `~/.local/share/cli-tutor/progress.json` after each correct answer.
- If the file doesn't exist or is malformed, start fresh (warn, don't panic).
- Progress is a `HashMap<String, ModuleProgress>` where the key is the module name (e.g. `"grep"`).

---

## Coding Standards

- **Rust edition:** 2021
- **Error type:** Use `anyhow::Result` for fallible functions in application code. Use `thiserror` for library-style error enums in `executor.rs` and `matcher.rs`.
- **Formatting:** `cargo fmt` — always. CI will enforce this.
- **Linting:** `cargo clippy -- -D warnings` — fix all warnings, never suppress with `#[allow]` without a comment explaining why.
- **Tests:** Integration tests in `tests/`, unit tests inline as `#[cfg(test)]` modules.
- **Minimum terminal size:** 80×24. If terminal is smaller, display a resize prompt and block all other rendering.

---

## Key Dependencies (Cargo.toml)

```toml
[dependencies]
ratatui = "0.29"
crossterm = "0.28"
serde = { version = "1", features = ["derive"] }
toml = "0.8"
serde_json = "1"
anyhow = "1"
thiserror = "1"

[dev-dependencies]
# no extra test deps needed; use std::process for integration tests
```

Do not add dependencies without a clear reason. Prefer std over external crates for simple tasks.

---

## Content Format

Exercise TOML files live in `content/<command>.toml`. See `PRD.md §7` for the full schema.
When writing or modifying content files:
- `match_mode` must be one of: `exact`, `normalized`, `sorted`, `regex`
- `difficulty` must be one of: `beginner`, `intermediate`, `advanced`
- Each exercise `id` must be globally unique and follow the pattern `<module>.<n>` (e.g. `grep.1`)
- Fixture filenames must not contain path separators

---

## Testing Requirements

- `executor.rs`: test timeout, stdout/stderr capture, env stripping, temp dir isolation
- `matcher.rs`: test all four match modes with passing and failing cases
- `progress.rs`: test load-missing-file (fresh start), load-corrupt-file (fresh start + warn), save, round-trip
- UI: no UI unit tests required (Ratatui widgets are hard to unit test); rely on manual + integration tests
- Exercise content: at least one smoke test that deserializes every bundled TOML file successfully

---

## Git Conventions

- Branch naming: `feat/<feature-name>`, `fix/<bug-description>`, `content/<module-name>`
- Commit messages: `<type>(<scope>): <description>` — e.g. `feat(executor): add 3s timeout with channel`
- Never commit directly to `main`
- Every PR must pass `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test`

---

## What NOT to Do

- Do not add mouse event handling (not required for v1.0)
- Do not use `thread::sleep` in the main event loop
- Do not use `tokio` or any async runtime — this app is synchronous I/O only
- Do not store rendered strings in state — compute them in the `ui/` layer at draw time
- Do not hardcode terminal colors — use Ratatui's `Style` with named colors so themes work
- Do not write fixture files outside the session temp directory
