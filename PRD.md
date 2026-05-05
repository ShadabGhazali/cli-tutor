# CLI Tutor — Product Requirements Document

**Version:** 1.0  
**Status:** Draft  
**Language/Stack:** Rust + Ratatui + Crossterm  

---

## 1. Overview

CLI Tutor is a terminal user interface (TUI) application for learning Unix/Linux command-line tools interactively. For each command (e.g. `grep`, `awk`, `sed`, `find`, `xargs`), the app provides:

1. A brief introduction and conceptual overview
2. A set of worked-out examples with explanations
3. A set of graded exercises where the user types real shell commands and the app validates output against expected output

The reference inspiration is [learnbyexample/TUI-apps/GrepExercises](https://github.com/learnbyexample/TUI-apps/tree/main/GrepExercises), which uses Python + Textual. CLI Tutor re-implements and extends this concept in **Rust** for maximum performance, minimal CPU usage, and a single statically-linked binary with no runtime dependencies.

---

## 2. Goals

- **Learn by doing:** Users type real shell commands into an input field; the app executes them in a sandboxed working directory and compares stdout against expected output.
- **Structured curriculum:** Each command is organized into a module. Within a module, content flows: intro → worked examples → exercises (easy → hard).
- **Zero friction:** Ships as a single binary. No Python, no npm, no virtual environments.
- **Minimal resource usage:** CPU usage should be near zero when idle. Rendering only on state change (event-driven, no polling/tick loops).
- **Extensible content:** Command modules and their exercises are defined in bundled TOML/JSON data files. Adding a new command module requires no code changes — only a new data file.

---

## 3. Non-Goals

- This is not a general-purpose shell emulator or terminal multiplexer.
- This is not a web app or Electron app.
- Mouse support is a stretch goal, not a requirement.
- Windows support is a stretch goal. Primary targets are Linux and macOS.

---

## 4. Technology Stack

| Concern | Choice | Rationale |
|---|---|---|
| Language | **Rust** | Zero-cost abstractions, no GC, single binary output, memory safe |
| TUI framework | **Ratatui** | Industry standard Rust TUI; immediate-mode rendering; minimal overhead |
| Terminal backend | **Crossterm** | Cross-platform (Linux/macOS/Windows), well-maintained |
| Command execution | **std::process::Command** | Built-in, no deps; runs commands in a sandboxed temp dir |
| Content format | **TOML** | Human-readable, easy to author, serde-friendly |
| Serialization | **serde + toml** | Zero-overhead deserialization at startup |
| Progress persistence | **JSON file in ~/.local/share/cli-tutor/** | Simple, portable, no DB dependency |

---

## 5. Application Layout

```
┌─────────────────────────────────────────────────────────────────┐
│  CLI Tutor  │  grep (3/12) ✓✓✗✓…  │  [Tab] Examples  [?] Help  │  ← Header
├──────────────────────────┬──────────────────────────────────────┤
│                          │                                      │
│   COMMAND LIST           │   CONTENT PANE                       │
│   ──────────             │   ──────────                         │
│ > grep        [12 ex]    │   [Intro / Example / Exercise text]  │
│   awk         [ 8 ex]    │                                      │
│   sed         [10 ex]    │   ┌──────────────────────────────┐   │
│   find        [ 9 ex]    │   │  $ _                         │   │
│   xargs       [ 6 ex]    │   └──────────────────────────────┘   │
│   cut         [ 5 ex]    │                                      │
│   sort        [ 6 ex]    │   Output:                            │
│   uniq        [ 4 ex]    │   ┌──────────────────────────────┐   │
│   tr          [ 4 ex]    │   │                              │   │
│                          │   │                              │   │
│                          │   └──────────────────────────────┘   │
│                          │                                      │
│                          │   [← Prev]  Exercise 3/12  [Next →]  │
├──────────────────────────┴──────────────────────────────────────┤
│  ↑↓ Navigate │ Enter: Submit │ Tab: Toggle view │ q: Quit       │  ← Footer
└─────────────────────────────────────────────────────────────────┘
```

### Panel Breakdown

**Left Panel — Command List (25% width)**
- Scrollable list of available command modules
- Each entry shows: command name + exercise count + completion badge
- Highlighted selection drives the content pane

**Right Panel — Content Pane (75% width)**
- Has three sub-views toggled by Tab or automatic progression:
  - **Intro view:** Scrollable markdown-like text with the command's overview
  - **Examples view:** A read-only list of worked examples with descriptions
  - **Exercise view:** The active learning mode (see below)

**Exercise View (within the content pane)**
- Question text at the top (scrollable)
- A sample file listing if the exercise uses input files
- A command input field (`$ `)
- An output display panel showing stdout/stderr of the user's last command
- Status line: correct ✓ / wrong ✗ / hint available
- Navigation: Prev / Next exercise buttons (keyboard)

---

## 6. User Flows

### 6.1 First Launch
1. App opens, selects the first command module (`grep`) automatically.
2. Content pane shows the **Intro** view.
3. User presses Tab or `e` to jump to exercises.

### 6.2 Working an Exercise
1. Exercise question is displayed with any relevant file contents.
2. User types a shell command in the input field and presses Enter.
3. App executes the command in a sandboxed temp directory populated with the exercise's fixture files.
4. Stdout is captured and compared (exact match or normalized match) against expected output.
5. If correct: green ✓, progress saved, option to go to next.
6. If wrong: red ✗, output is shown for diffing. User can retry freely.
7. User can press `h` to reveal a hint (one level at a time).
8. User can press `s` to reveal the reference solution.

### 6.3 Navigation
- `↑`/`↓` in the command list: change module
- `Tab`: cycle Intro → Examples → Exercises within a module
- `n`/`p` or `→`/`←`: next/previous exercise
- `q` or `Ctrl+C`: quit

---

## 7. Content Data Model

Each command module is a TOML file bundled into the binary at compile time via `include_str!` or `rust-embed`.

```toml
[module]
name = "grep"
description = "Search for patterns in files using regular expressions"
version = 1

[intro]
text = """
`grep` searches for lines matching a pattern...
(full intro text here)
"""

[[examples]]
title = "Basic pattern search"
description = "Find lines containing 'error' in a log file"
command = "grep 'error' app.log"
output = """
2024-01-15 error: connection refused
2024-01-16 error: timeout
"""

[[examples]]
title = "Case-insensitive search"
command = "grep -i 'ERROR' app.log"
output = "..."

[[exercises]]
id = "grep.1"
difficulty = "beginner"
question = """
The file `fruits.txt` contains one fruit per line.
Find all lines that contain the word 'mango'.
"""
expected_output = """
mango
mango chutney
"""
hints = [
  "Use grep with the pattern 'mango'",
  "Try: grep 'mango' fruits.txt"
]
solution = "grep 'mango' fruits.txt"
match_mode = "exact"   # or "normalized" (strips trailing whitespace/blank lines)

[[exercises.fixtures]]
filename = "fruits.txt"
content = """
apple
mango
banana
mango chutney
grape
"""
```

---

## 8. Command Execution & Sandboxing

- Each exercise execution creates (or reuses) a temp directory scoped to the session.
- Fixture files are written to that directory before executing the user's command.
- Commands are executed via `sh -c "<user_input>"` with a configurable timeout (default: 3 seconds).
- Stdout and stderr are captured separately.
- The working directory of the subprocess is set to the fixture temp dir.
- Environment is minimal: `PATH` is preserved; `HOME`, `USER`, `TERM` are set; everything else is stripped.
- No network access is granted (enforced via minimal env; hard sandboxing like seccomp is a future enhancement).

### Output Matching Modes
| Mode | Behaviour |
|---|---|
| `exact` | Byte-for-byte match of stdout |
| `normalized` | Strips trailing whitespace per line, removes trailing blank lines |
| `sorted` | Lines are sorted before comparison (for commands whose output order is non-deterministic) |
| `regex` | Expected output is a regex that must match stdout |

---

## 9. Progress Persistence

Progress is stored in `~/.local/share/cli-tutor/progress.json`:

```json
{
  "grep": {
    "completed": ["grep.1", "grep.2", "grep.5"],
    "attempted": ["grep.3"]
  },
  "awk": {
    "completed": [],
    "attempted": []
  }
}
```

Progress is written after each correct answer. It is never deleted automatically.

---

## 10. Initial Command Modules (v1.0)

| Module | Exercises | Topics Covered |
|---|---|---|
| `grep` | 12 | basic patterns, flags (-i, -v, -r, -n, -c, -l), regex, ripgrep compat |
| `awk` | 10 | field splitting, print, conditions, BEGIN/END, arithmetic |
| `sed` | 10 | substitution, deletion, address ranges, in-place |
| `find` | 9 | name/type/mtime/size filters, -exec, -print0 |
| `xargs` | 6 | basic, -I, -0, parallel, combining with find |
| `cut` | 5 | -d, -f, -c, combining fields |
| `sort` | 6 | -n, -r, -k, -u, -t, stability |
| `uniq` | 4 | -c, -d, -u, combining with sort |
| `tr` | 4 | character sets, -d, -s, squeeze |

Total: **~66 exercises** at launch.

---

## 11. Keyboard Reference

| Key | Action |
|---|---|
| `↑` / `↓` | Navigate command list |
| `Tab` | Cycle content view (Intro → Examples → Exercises) |
| `→` / `n` | Next exercise |
| `←` / `p` | Previous exercise |
| `Enter` | Submit command |
| `h` | Show next hint |
| `s` | Show solution |
| `r` | Reset exercise (clear output, re-write fixtures) |
| `Ctrl+L` | Clear output panel |
| `?` | Toggle help overlay |
| `q` / `Ctrl+C` | Quit |

---

## 12. Minimum Terminal Size

- **Minimum:** 80 columns × 24 rows
- **Recommended:** 120 columns × 36 rows
- If terminal is too small, display a "Please resize your terminal" message and block input.

---

## 13. Error Handling

- Command timeout: show "Command timed out after 3s"
- Command not found: show stderr verbatim
- Permission denied: show stderr verbatim
- Progress file corrupt: warn and start fresh (do not crash)
- Fixture write failure: show error and skip execution

---

## 14. Performance Constraints

- Idle CPU usage: **< 0.1%** (event-driven rendering, no tick loops)
- Cold start time: **< 50ms** (all content is bundled at compile time)
- Command execution latency: bounded only by the user's command (3s timeout)
- Binary size: target **< 5MB** stripped

---

## 15. Project File Structure

```
cli-tutor/
├── Cargo.toml
├── Cargo.lock
├── PRD.md
├── CLAUDE.md
├── AGENTS.md
├── features/
│   ├── navigation.feature.yaml
│   ├── exercise_runner.feature.yaml
│   ├── text_input.feature.yaml
│   ├── content_display.feature.yaml
│   ├── progress_tracking.feature.yaml
│   └── command_modules.feature.yaml
├── src/
│   ├── main.rs
│   ├── app.rs            # App state, event loop
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── layout.rs     # Root layout rendering
│   │   ├── command_list.rs
│   │   ├── content_pane.rs
│   │   ├── exercise_view.rs
│   │   ├── input_bar.rs
│   │   └── help_overlay.rs
│   ├── executor.rs       # subprocess sandboxing + output capture
│   ├── matcher.rs        # output comparison logic
│   ├── progress.rs       # load/save progress.json
│   └── content/
│       ├── mod.rs        # content loader, module registry
│       └── types.rs      # Module, Example, Exercise structs
├── content/
│   ├── grep.toml
│   ├── awk.toml
│   ├── sed.toml
│   ├── find.toml
│   ├── xargs.toml
│   ├── cut.toml
│   ├── sort.toml
│   ├── uniq.toml
│   └── tr.toml
└── tests/
    ├── executor_tests.rs
    ├── matcher_tests.rs
    └── progress_tests.rs
```
