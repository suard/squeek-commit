# Squeek Commit

A simple CLI tool written in Rust that squashes multiple commits on a Git branch compared to a base branch. It's useful for cleaning up commit history before opening a pull request or merging into the main branch.

## 📦 Features

This tool:

- Locates a Git repository based on a provided path
- Counts the number of commits between the current branch and a specified base branch (e.g. `main`)
- Performs a **soft reset** to the commit before the first unique commit compared to the base branch
- Stages all changes (`git add .`)
- Creates a new commit with either a custom message or a reused message from an earlier commit
- Supports a **dry-run** mode that simulates the process without applying changes

## 🛠️ Requirements

- A Git repository
- Rust + Cargo installed
- Existing Git branches (e.g. `main` and a feature branch)

## 🚀 Usage & Example

### Option 1
Run the tool from the command line. It uses [`clap`](https://docs.rs/clap/latest/clap/) to handle CLI arguments.

```bash
cargo run -- --path . --main main --message "My squashed commit"
```

### Option 2
Use `copy-squeek-commit.sh` to copy the squeek-commit binary to `/usr/local/bin`. Then just squeek-commit on the command line.

```bash
squeek-commit --path . --main main --message "My squashed commit"
```

## 🧾 Arguments

The following command-line arguments are supported:

| Argument        | Type     | Description                                                                 | Required | Default                           |
|-----------------|----------|-----------------------------------------------------------------------------|--|-----------------------------------|
| `--path`        | `String` | Path to the Git repository                                                  | ❌ No  | `.`                                |
| `--main`        | `String` | Name of the base branch to compare against (e.g. `main`, `develop`)         | ❌ No | `main`                            |
| `--message`     | `String` | Custom commit message for the new squashed commit                          | ❌ No | Uses message from previous commit |
| `--dry`         | `Flag`   | Run in dry-run mode (no actual reset or commit will be made)                | ❌ No | `false`                           |

