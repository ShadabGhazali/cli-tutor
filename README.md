# cli-tutor

A terminal app for learning Unix command-line tools by doing. Type real shell commands, get instant feedback.

Covers: `grep`, `awk`, `sed`, `find`, `xargs`, `cut`, `sort`, `uniq`, `tr` — ~66 exercises across beginner to advanced.

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
| `Tab` | Intro → Examples → Exercises |
| `PgUp` `PgDn` | Scroll |
| `q` | Quit |

**Exercises**

| Key | Action |
|-----|--------|
| `Enter` | Submit command |
| `↑` `↓` | Scroll output |
| `←` `→` | Move cursor |
| `Ctrl+N` `Ctrl+P` | Next / prev exercise |
| `Ctrl+T` | Hint |
| `Ctrl+S` | Solution |
| `Ctrl+F` | Show files |
| `Ctrl+R` | Reset |
| `Ctrl+L` | Clear output |
| `Esc` | Back to browse |
| `Ctrl+C` | Quit |

Progress is saved to `~/.local/share/cli-tutor/progress.json`.

## Stack

Rust + [Ratatui](https://github.com/ratatui/ratatui) + Crossterm. Single binary, no runtime dependencies.
