# Roadmap

## ✅ Completed

- [x] Implement `App::dispatch` logic in `src/app.rs`.
- [x] Integrate `clap` for professional CLI argument handling.
- [x] Add `--all` and `--status` options to `list` command with case-insensitive status parsing.

## 🎯 Short-Term Goals

- [ ] Add `--status` and `--description` options to `update <id>` command.
- [ ] Add color support using `colored` or `termcolor` crates.
- [ ] Add a "clear all" command.

## 🚀 Long-Term Goals

- [ ] Support for different storage backends (SQLite, YAML).
- [ ] Interactive "Guided" mode for task creation.
- [ ] Export tasks to CSV/Markdown formats.
- [ ] GitHub Actions for CI (linting and testing).
- [ ] Documentation site using `Bookworm` or `MkDocs`.
