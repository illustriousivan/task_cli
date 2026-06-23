mod commands;

pub use commands::Commands;

use clap::Parser;
use task_cli::core::status::parse_status;
use task_cli::core::tasks::Status;
use task_cli::storage::{Storage, StorageError};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppError {
    TaskNotFound(u32),
    EmptyStorage,
    InvalidDescription,
}

impl From<StorageError> for AppError {
    fn from(err: StorageError) -> Self {
        match err {
            StorageError::TaskNotFound(id) => AppError::TaskNotFound(id),
            StorageError::EmptyStorage => AppError::EmptyStorage,
        }
    }
}

pub struct App {
    storage: Box<dyn Storage<Error = StorageError>>,
}

impl App {
    pub fn new(storage: Box<dyn Storage<Error = StorageError>>) -> Self {
        App { storage }
    }

    pub fn dispatch(&mut self, command: Commands) -> Result<(), AppError> {
        match command {
            Commands::Create { description } => {
                let desc = match description {
                    None => {
                        eprintln!("Nothing has changed.");
                        eprintln!("Usage: create --description \"text\"");
                        return Ok(());
                    }
                    Some(s) if s.trim().is_empty() => return Err(AppError::InvalidDescription),
                    Some(s) => s,
                };
                self.storage.create(desc)?;
                Ok(())
            }
            Commands::Remove { id } => {
                self.storage.remove(id)?;
                Ok(())
            }
            Commands::Update {
                id,
                description,
                status,
            } => {
                let (desc_changed, status_changed) = (description.is_some(), status.is_some());
                if !desc_changed && !status_changed {
                    eprintln!("Nothing has changed.");
                    eprintln!(
                        "Usage: update <id> --description \"text\" or --status [todo|in-progress|done]"
                    );
                    return Ok(());
                }

                let mut task = self.storage.get(id)?;

                if let Some(desc) = description {
                    task.description = desc;
                }

                if let Some(status_str) = status {
                    let parsed_status =
                        parse_status(&status_str).map_err(|_| AppError::EmptyStorage)?;
                    task.status = parsed_status;
                }

                self.storage.update(task)?;
                Ok(())
            }
            Commands::List { all, status } => {
                let tasks = if all {
                    self.storage.list()
                } else if let Some(ref s) = status {
                    let parsed_status =
                        parse_status(s.as_str()).map_err(|_| AppError::EmptyStorage)?;
                    self.storage.list_by_status(parsed_status)
                } else {
                    self.storage
                        .list()
                        .into_iter()
                        .filter(|t| t.status != Status::Done)
                        .collect()
                };
                if tasks.is_empty() {
                    return Ok(());
                }
                for task in tasks {
                    let status_str = match task.status {
                        Status::Todo => "to-do",
                        Status::InProgress => "in progress",
                        Status::Done => "done",
                    };
                    println!("{}: ({}) {}", task.id, status_str, task.description);
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;
    use task_cli::core::tasks::{Status, Task};
    use task_cli::storage::StorageError;

    pub struct MockStorage {
        tasks: Vec<Task>,
        list_called_times: Rc<RefCell<u8>>,
    }

    impl Storage for MockStorage {
        type Error = StorageError;

        fn create(&mut self, description: String) -> Result<u32, Self::Error> {
            let id = (self.tasks.len() + 1) as u32;
            self.tasks.push(Task {
                id,
                description,
                status: Status::Todo,
            });
            Ok(id)
        }

        fn get(&self, id: u32) -> Result<Task, Self::Error> {
            if self.tasks.is_empty() {
                return Err(StorageError::EmptyStorage);
            }
            self.tasks
                .iter()
                .find(|t| t.id == id)
                .cloned()
                .ok_or(StorageError::TaskNotFound(id))
        }

        fn update(&mut self, task: Task) -> Result<(), Self::Error> {
            if self.tasks.is_empty() {
                return Err(StorageError::EmptyStorage);
            }
            let existing = self.tasks.iter_mut().find(|t| t.id == task.id);
            match existing {
                Some(t) => *t = task,
                None => return Err(StorageError::TaskNotFound(task.id)),
            }
            Ok(())
        }

        fn remove(&mut self, id: u32) -> Result<(), Self::Error> {
            if self.tasks.is_empty() {
                return Err(StorageError::EmptyStorage);
            }
            let pos = self.tasks.iter().position(|t| t.id == id);
            match pos {
                Some(i) => {
                    self.tasks.remove(i);
                    Ok(())
                }
                None => Err(StorageError::TaskNotFound(id)),
            }
        }

        fn list(&self) -> Vec<Task> {
            *self.list_called_times.borrow_mut() += 1;
            self.tasks.clone()
        }

        fn list_by_status(&self, status: Status) -> Vec<Task> {
            self.tasks
                .iter()
                .filter(|t| t.status == status)
                .cloned()
                .collect()
        }
    }

    impl MockStorage {
        pub fn new() -> Self {
            MockStorage {
                tasks: Vec::new(),
                list_called_times: Rc::new(RefCell::new(0)),
            }
        }
    }

    #[test]
    fn app_dispatch_creates_one_task() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let command = Commands::Create {
            description: Some("Task".into()),
        };
        let result = app.dispatch(command);
        assert!(result.is_ok());
        assert_eq!(
            app.storage.list(),
            vec![Task {
                id: 1,
                description: "Task".into(),
                status: Status::Todo,
            }]
        );
    }

    #[test]
    fn app_dispatch_creates_sequential_id_tasks() {
        let mut app = App::new(Box::new(MockStorage::new()));
        for i in 1..=3 {
            let _result = app.dispatch(Commands::Create {
                description: Some(format!("Task {}", i)),
            });
        }
        for (i, task) in app.storage.list().iter().enumerate() {
            assert_eq!((i + 1) as u32, task.id);
        }
    }

    #[test]
    fn app_dispatch_removes_one_task() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let _result = app.dispatch(Commands::Create {
            description: Some("Remove me".into()),
        });
        let command = Commands::Remove { id: 1 };
        let result = app.dispatch(command);
        assert!(result.is_ok());
        assert_eq!(app.storage.list(), vec![]);
    }

    #[test]
    fn app_dispatch_trying_to_remove_from_empty_storage_returns_empty_storage_error() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let command = Commands::Remove { id: 1 };
        let result = app.dispatch(command);
        assert!(result.is_err());
        assert_eq!(result, Err(AppError::EmptyStorage));
    }

    #[test]
    fn app_dispatch_trying_to_remove_invalid_id_returns_task_not_found_error() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let _result = app.dispatch(Commands::Create {
            description: Some("Task".into()),
        });
        let command = Commands::Remove { id: 2 };
        let result = app.dispatch(command);
        assert!(result.is_err());
        assert_eq!(result, Err(AppError::TaskNotFound(2)));
    }

    #[test]
    fn app_dispatch_update_no_options_prints_nothing_changed() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let _ = app.dispatch(Commands::Create {
            description: Some("Task".into()),
        });
        let command = Commands::Update {
            id: 1,
            description: None,
            status: None,
        };
        let result = app.dispatch(command);
        assert!(result.is_ok());
    }

    #[test]
    fn app_dispatch_update_only_description_updates_description_preserves_status() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let _ = app.dispatch(Commands::Create {
            description: Some("Original".into()),
        });
        let command = Commands::Update {
            id: 1,
            description: Some("Updated".into()),
            status: None,
        };
        let result = app.dispatch(command);
        assert!(result.is_ok());
        assert_eq!(
            app.storage.list(),
            vec![Task {
                id: 1,
                description: "Updated".into(),
                status: Status::Todo,
            }]
        );
    }

    #[test]
    fn app_dispatch_update_only_status_updates_status_preserves_description() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let _ = app.dispatch(Commands::Create {
            description: Some("Task".into()),
        });
        let command = Commands::Update {
            id: 1,
            description: None,
            status: Some("done".into()),
        };
        let result = app.dispatch(command);
        assert!(result.is_ok());
        assert_eq!(
            app.storage.list(),
            vec![Task {
                id: 1,
                description: "Task".into(),
                status: Status::Done,
            }]
        );
    }

    #[test]
    fn app_dispatch_update_both_description_and_status_updates_both() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let _ = app.dispatch(Commands::Create {
            description: Some("Original".into()),
        });
        let command = Commands::Update {
            id: 1,
            description: Some("Updated".into()),
            status: Some("done".into()),
        };
        let result = app.dispatch(command);
        assert!(result.is_ok());
        assert_eq!(
            app.storage.list(),
            vec![Task {
                id: 1,
                description: "Updated".into(),
                status: Status::Done,
            }]
        );
    }

    #[test]
    fn app_dispatch_update_invalid_status_returns_error() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let _ = app.dispatch(Commands::Create {
            description: Some("Task".into()),
        });
        let command = Commands::Update {
            id: 1,
            description: None,
            status: Some("invalid".into()),
        };
        let result = app.dispatch(command);
        assert!(result.is_err());
    }

    #[test]
    fn app_dispatch_update_nonexistent_id_returns_task_not_found() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let _ = app.dispatch(Commands::Create {
            description: Some("Task".into()),
        });
        let command = Commands::Update {
            id: 99,
            description: Some("Updated".into()),
            status: None,
        };
        let result = app.dispatch(command);
        assert!(result.is_err());
        assert_eq!(result, Err(AppError::TaskNotFound(99)));
    }

    #[test]
    fn app_dispatch_update_empty_storage_returns_error() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let command = Commands::Update {
            id: 1,
            description: Some("Updated".into()),
            status: None,
        };
        let result = app.dispatch(command);
        assert!(result.is_err());
        assert_eq!(result, Err(AppError::EmptyStorage));
    }

    #[test]
    fn app_dispatch_calls_storage_list_method() {
        let mock = MockStorage::new();
        let list_called_times = mock.list_called_times.clone();
        let mut app = App::new(Box::new(mock));
        assert_eq!(*list_called_times.borrow(), 0);
        let result = app.dispatch(Commands::List {
            all: false,
            status: None,
        });
        assert!(result.is_ok());
        assert_eq!(*list_called_times.borrow(), 1);
    }

    #[test]
    fn app_dispatch_list_default_shows_todo_and_in_progress_but_not_done() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let _ = app.dispatch(Commands::Create {
            description: Some("Todo task".into()),
        });
        let command = Commands::List {
            all: false,
            status: None,
        };
        let result = app.dispatch(command);
        assert!(result.is_ok());
    }

    #[test]
    fn app_dispatch_list_all_shows_everything() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let command = Commands::List {
            all: true,
            status: None,
        };
        let result = app.dispatch(command);
        assert!(result.is_ok());
    }

    #[test]
    fn app_dispatch_list_with_status_uses_list_by_status() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let command = Commands::List {
            all: false,
            status: Some("todo".into()),
        };
        let result = app.dispatch(command);
        assert!(result.is_ok());
    }

    #[test]
    fn app_dispatch_list_with_status_done_uses_list_by_status() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let command = Commands::List {
            all: false,
            status: Some("done".into()),
        };
        let result = app.dispatch(command);
        assert!(result.is_ok());
    }

    #[test]
    fn app_dispatch_create_empty_description_returns_invalid_description_error() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let command = Commands::Create {
            description: Some("".into()),
        };
        let result = app.dispatch(command);
        assert!(result.is_err());
        assert_eq!(result, Err(AppError::InvalidDescription));
    }

    #[test]
    fn app_dispatch_create_whitespace_description_returns_invalid_description_error() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let command = Commands::Create {
            description: Some("   ".into()),
        };
        let result = app.dispatch(command);
        assert!(result.is_err());
        assert_eq!(result, Err(AppError::InvalidDescription));
    }

    #[test]
    fn app_dispatch_create_without_args_prints_usage_message() {
        let mut app = App::new(Box::new(MockStorage::new()));
        let command = Commands::Create { description: None };
        let result = app.dispatch(command);
        assert!(result.is_ok());
    }
}
