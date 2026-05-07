# cli-tutor

[![crates.io](https://img.shields.io/crates/v/cli-tutor.svg)](https://crates.io/crates/cli-tutor)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Learn Unix command-line tools by typing real commands — not reading about them.

Pick a tool, read the intro, try the examples, then solve exercises in a live shell.
Each command you type is actually executed and checked against the expected output.

![cli-tutor demo](assets/demo.gif)

---

## Install

**Homebrew (macOS/Linux)**
```sh
brew install ShadabGhazali/cli-tutor/cli-tutor
```

**Cargo**
```sh
cargo install cli-tutor
```

**Pre-built binary (no Rust required)**

1. Go to the [Releases](https://github.com/ShadabGhazali/cli-tutor/releases) page and download the archive for your platform:
   - `cli-tutor-aarch64-apple-darwin.tar.gz` — macOS Apple Silicon (M1/M2/M3)
   - `cli-tutor-x86_64-apple-darwin.tar.gz` — macOS Intel
   - `cli-tutor-x86_64-unknown-linux-musl.tar.gz` — Linux x86_64
   - `cli-tutor-x86_64-pc-windows-msvc.zip` — Windows x86_64

2. Extract and run:

```sh
# macOS / Linux
tar xzf cli-tutor-*.tar.gz
chmod +x cli-tutor
./cli-tutor

# macOS — if blocked by Gatekeeper, remove the quarantine flag first
xattr -d com.apple.quarantine cli-tutor
./cli-tutor
```

```powershell
# Windows — extract the zip, then run in a terminal
.\cli-tutor.exe
```

<details>
<summary>Update an existing install</summary>

```sh
# Homebrew
brew upgrade ShadabGhazali/cli-tutor/cli-tutor

# Cargo
cargo install cli-tutor --force
```
</details>

<details>
<summary>Build from source</summary>

```sh
cargo build --release
./target/release/cli-tutor
```

Requires a terminal at least 80×24.
</details>

---

## Modules

Sorted from most-used daily foundations through dev tools and into thematic real-world workflows.

### Foundations — navigate and view

| Tool | What it does | Exercises |
|------|-------------|-----------|
| `ls` | List directory contents, spot hidden files, sort by time or size | 6 |
| `cat` | Display and concatenate files — the start of every pipeline | 5 |
| `head` | Preview the first N lines — check CSV headers, sample data | 5 |
| `tail` | View the last N lines, skip headers, monitor recent log entries | 6 |

### Search and filter

| Tool | What it does | Exercises |
|------|-------------|-----------|
| `grep` | Search files for patterns using regular expressions | 12 |
| `find` | Locate files by name, type, size, age, permissions | 9 |

### Transform and process

| Tool | What it does | Exercises |
|------|-------------|-----------|
| `cut` | Extract columns from delimited text | 5 |
| `sort` | Sort lines — alphabetically, numerically, by field | 6 |
| `uniq` | Remove or count duplicate adjacent lines | 4 |
| `wc` | Count lines, words, and bytes | 6 |
| `tr` | Translate or delete individual characters | 4 |
| `sed` | Stream editor — substitute, delete, transform lines | 10 |
| `awk` | Field-based processing language — extract, filter, aggregate | 10 |

### Combine and compare

| Tool | What it does | Exercises |
|------|-------------|-----------|
| `paste` | Merge lines from multiple files side by side | 4 |
| `tee` | Fork a pipeline — write to a file AND pass through to stdout | 4 |
| `diff` | Compare two files and show what changed | 5 |

### Archive, permissions, and utilities

| Tool | What it does | Exercises |
|------|-------------|-----------|
| `xargs` | Build and run commands from standard input | 6 |
| `tar` | Create, list, and extract archive files | 5 |
| `chmod` | Change file permissions | 6 |
| `bc` | Arbitrary-precision arithmetic — percentages, averages in scripts | 4 |

### Dev tools

| Tool | What it does | Exercises |
|------|-------------|-----------|
| `git` | Version control — init, commit, branch, stash, diff | 10 |
| `jq` | Query and transform JSON from the command line | 8 |
| `make` | Automate build steps and tasks with Makefiles | 5 |

### Real-world workflows

These modules teach you to **combine tools in pipelines** to solve complete engineering tasks — not just individual command syntax.

| Module | What it covers | Exercises |
|--------|---------------|-----------|
| `log-processing` | Analyze Apache/app logs: count requests, find errors, top-N IPs, error rates | 8 |
| `text-processing` | Transform CSV data, extract fields, calculate stats, process config files | 8 |

**161 exercises** — beginner to advanced. Progress is saved automatically.

---

## Keys

<details>
<summary>Show all keybindings</summary>

**Browsing (Intro / Examples views)**

| Key | Action |
|-----|--------|
| `↑` `↓` | Switch module |
| `Tab` | Cycle views: Intro → Examples → Exercise → Free Practice |
| `PgUp` `PgDn` | Scroll content |
| `/` | Fuzzy search modules |
| `d` | Cycle difficulty filter (All → Beginner → Intermediate → Advanced) |
| `P` | Progress summary |
| `q` | Quit |

**Exercise view**

| Key | Action |
|-----|--------|
| `Enter` | Submit command |
| `↑` `↓` | Browse command history |
| `PgUp` `PgDn` | Scroll output |
| `←` `→` | Move cursor |
| `Ctrl+←` `Ctrl+→` | Jump by word |
| `Ctrl+N` `Ctrl+P` | Next / previous exercise |
| `Ctrl+T` | Reveal next hint |
| `Ctrl+S` | Show / hide solution |
| `Ctrl+F` | Toggle file viewer |
| `Ctrl+R` | Reset exercise |
| `Ctrl+L` | Clear output |
| `Esc` | Back to browse |

**Free Practice view**

| Key | Action |
|-----|--------|
| `Enter` | Run command |
| `↑` `↓` | Browse command history |
| `PgUp` `PgDn` | Scroll output |
| `Ctrl+L` | Clear output |
| `Esc` | Back to browse |

**Anywhere**

| Key | Action |
|-----|--------|
| `Shift+P` | Toggle progress overlay |
| `?` | Toggle help |
| `Ctrl+C` | Quit |

</details>

---

## CLI flags

```sh
cli-tutor --version
cli-tutor --no-color
cli-tutor --completions bash | zsh | fish
```

<details>
<summary>Install shell completions</summary>

```sh
# bash
cli-tutor --completions bash >> ~/.bash_completion

# zsh
cli-tutor --completions zsh > ~/.zsh/completions/_cli-tutor

# fish
cli-tutor --completions fish > ~/.config/fish/completions/cli-tutor.fish
```
</details>

---

## Config

Optional config at `~/.config/cli-tutor/config.toml` (respects `$XDG_CONFIG_HOME`):

```toml
no_color        = false   # disable all colour styling
timed_challenge = false   # show solve time and personal best on correct answers
skip_completed  = false   # auto-skip already-solved exercises during navigation
default_module  = "grep"  # open on this module instead of the first in the list
```

A missing or corrupt config file is silently ignored — defaults apply.

---

## Stack

[Rust](https://www.rust-lang.org) + [Ratatui](https://github.com/ratatui/ratatui) + [Crossterm](https://github.com/crossterm-rs/crossterm). Single binary, no runtime dependencies.
