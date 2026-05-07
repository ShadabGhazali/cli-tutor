# cli-tutor

A terminal app for learning Unix command-line tools by doing. Type real shell commands, get instant feedback.

Covers: `grep`, `awk`, `sed`, `find`, `xargs`, `cut`, `sort`, `uniq`, `tr`, `wc`, `tar`, `chmod` — 83 exercises across beginner to advanced.

## Install

**Homebrew (macOS/Linux)**
```sh
brew install ShadabGhazali/cli-tutor/cli-tutor
```

**Cargo**
```sh
cargo install cli-tutor
```

Or grab a pre-built binary from [Releases](https://github.com/ShadabGhazali/cli-tutor/releases).

## Update

**Homebrew**
```sh
brew upgrade ShadabGhazali/cli-tutor/cli-tutor
```

**Cargo**
```sh
cargo install cli-tutor --force
```

## Run from source

```sh
cargo run
```

Or build a release binary:

```sh
cargo build --release
./target/release/cli-tutor
```

Requires a terminal at least 80×24.

## Keys

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

## CLI flags

```sh
cli-tutor --version
cli-tutor --no-color
cli-tutor --completions bash | zsh | fish
```

To install shell completions:

```sh
# bash
cli-tutor --completions bash >> ~/.bash_completion

# zsh
cli-tutor --completions zsh > ~/.zsh/completions/_cli-tutor

# fish
cli-tutor --completions fish > ~/.config/fish/completions/cli-tutor.fish
```

## Config file

Optional config at `~/.config/cli-tutor/config.toml` (respects `$XDG_CONFIG_HOME`):

```toml
no_color        = false   # disable all colour styling
timed_challenge = false   # show solve time and personal best on correct answers
skip_completed  = false   # auto-skip already-solved exercises during navigation
default_module  = "grep"  # open on this module instead of the first in the list
```

A missing or corrupt config file is silently ignored — defaults apply.

## Progress

Progress is saved to `~/.local/share/cli-tutor/progress.json` after each correct answer.

## Stack

Rust + [Ratatui](https://github.com/ratatui/ratatui) + Crossterm. Single binary, no runtime dependencies.
