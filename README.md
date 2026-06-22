# Task CLI

A lightweight, robust command-line interface (CLI) tool built with Rust for
efficient task management. This project provides a clean way to organize your
daily activities, track progress, and persist your data using a local JSON
storage system.

## 🚀 Features

- **Create Tasks**: Easily add new tasks with descriptions.
- **List Tasks**: View tasks filtered by status, or show all tasks with `--all`.
  - Default: shows only `Todo` and `InProgress` tasks.
  - `task_cli list --status todo` — filter by specific status (case-insensitive).
- **Update Tasks**: Modify existing tasks by description, status, or both simultaneously.
  - `task_cli update <id> --description "New description"` — change the task description.
  - `task_cli update <id> --status done` — change the task status (case-insensitive).
  - Both options can be combined to update multiple fields at once.
- **Remove Tasks**: Delete completed or no longer needed tasks.
- **Status Tracking**: Categorize tasks into `Todo`, `InProgress`, or `Done`.
- **JSON Persistence**: Automatically saves your tasks to a local JSON file.

## 🛠 Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/)
- **Serialization**: [Serde](https://serde.rs/)
- **Storage**: Local JSON File System

## 📋 Installation

Prerequisites: [Rust and Cargo](https://rustup.rs/)

1. Clone the repository:

   ```bash
   git clone https://github.com/illustriousivan/task_cli.git
   cd task_cli
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the CLI:

   ```bash
   ./target/release/task_cli
   ```

## 📖 Usage Examples

- **Create a task**:

  ```bash
  task_cli create "Finish the portfolio project"
  ```

- **List all tasks**:

  ```bash
  task_cli list --all
  ```

- **List tasks by status** (case-insensitive, supports spaces and hyphens):

  ```bash
  task_cli list --status todo
  task_cli list --status "in-progress"
  task_cli list --status done
  ```

- **Update a task** (change description):

  ```bash
  task_cli update <id> --description "New description"
  ```

- **Update a task status**:

  ```bash
  task_cli update <id> --status done
  ```

- **Update both description and status at once**:

  ```bash
  task_cli update <id> --description "Completed task" --status done
  ```

- **Remove a task**:

  ```bash
  task_cli remove <id>
  ```

## 🏗 Architecture

- `src/core/`: Contains the core domain models (`tasks.rs`), command definitions (`commands.rs`), and status parsing (`status.rs`).
- `src/storage/`: Handles data persistence, specifically `json_storage.rs` for file I/O.
- `src/app.rs`: The main application logic and command dispatcher.

## 🤝 Contributing

Contributions are welcome! Please refer to the `CONTRIBUTING.md` for guidelines.

## 🗺 Roadmap

See [`ROADMAP.md`](./ROADMAP.md) for the full development plan.

---

*This project is a showcase of Rust proficiency, focusing on clean architecture,
trait-based polymorphism, and robust error handling.*
