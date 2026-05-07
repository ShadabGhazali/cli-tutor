# cli-tutor

[![crates.io](https://img.shields.io/crates/v/cli-tutor.svg)](https://crates.io/crates/cli-tutor)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Learn Unix command-line tools by typing real commands — not reading about them.

Pick a tool, read the intro, try the examples, then solve exercises in a live shell.
Each command you type is actually executed and checked against the expected output.

![cli-tutor screenshot](assets/screenshot.png)

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

**Pre-built binaries** — grab the latest from [Releases](https://github.com/ShadabGhazali/cli-tutor/releases) (macOS Intel/ARM, Linux, Windows).

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

| Tool | What it does | Exercises |
|------|-------------|-----------|
| `grep` | Search files for patterns | 12 |
| `awk` | Field-based text processing and transformation | 10 |
| `sed` | Stream editor — substitute, delete, insert lines | 10 |
| `find` | Locate files by name, type, size, permissions | 9 |
| `xargs` | Build and run commands from standard input | 6 |
| `cut` | Extract fields and columns from text | 5 |
| `sort` | Sort lines — alphabetically, numerically, by field | 6 |
| `uniq` | Remove or count duplicate adjacent lines | 4 |
| `tr` | Translate or delete individual characters | 4 |
| `wc` | Count lines, words, and bytes in files | 6 |
| `tar` | Create, list, and extract archive files | 5 |
| `chmod` | Change file permissions | 6 |

**83 exercises** — beginner to advanced. Progress is saved automatically.

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

Rust + [Ratatui](https://github.com/ratatui/ratatui) + Crossterm. Single binary, no runtime dependencies.
