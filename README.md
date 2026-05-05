# cli-tutor

A terminal app for learning Unix command-line tools by doing. Type real shell commands, get instant feedback.

Covers: `grep`, `awk`, `sed`, `find`, `xargs`, `cut`, `sort`, `uniq`, `tr` — ~66 exercises across beginner to advanced.

## Install

```sh
cargo install cli-tutor
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

| Key | Action |
|-----|--------|
| `↑` `↓` | Switch module |
| `Tab` | Intro → Examples → Exercises |
| `n` `p` | Next / prev exercise |
| `Enter` | Run command |
| `h` | Hint |
| `s` | Solution |
| `f` | Show files |
| `?` | Help |
| `q` | Quit |

Progress is saved to `~/.local/share/cli-tutor/progress.json`.

## Stack

Rust + [Ratatui](https://github.com/ratatui/ratatui) + Crossterm. Single binary, no runtime dependencies.
