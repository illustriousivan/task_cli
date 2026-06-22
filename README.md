# Task CLI

A lightweight, robust command-line interface (CLI) tool built with Rust for
efficient task management. This project provides a clean way to organize your
daily activities, track progress, and persist your data using a local JSON
storage system.

## 🚀 Features

- **Create Tasks**: Easily add new tasks with descriptions.
- **List Tasks**: View all your current tasks in a structured format.
- **Update Tasks**: Modify existing tasks (e.g., update descriptions).
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
  task_cli list
  ```

- **Update a task**:

  ```bash
  task_cli update <id> "New description"
  ```

- **Remove a task**:

  ```bash
  task_cli remove <id>
  ```

## 🏗 Architecture

- `src/core/`: Contains the core domain models (`tasks.rs`) and command definitions (`commands.rs`).
- `src/storage/`: Handles data persistence, specifically `json_storage.rs` for file I/O.
- `src/app.rs`: The main application logic and command dispatcher.

## 🤝 Contributing

Contributions are welcome! Please refer to the `CONTRIBUTING.md` for guidelines.

## 🗺 Roadmap

See [`ROADMAP.md`](./ROADMAP.md) for the full development plan.

---

*This project is a showcase of Rust proficiency, focusing on clean architecture,
trait-based polymorphism, and robust error handling.*
