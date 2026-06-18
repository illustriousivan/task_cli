use task_cli::core::commands::Commands;
use task_cli::storage::{Storage, StorageError};

#[derive(Debug, Clone, PartialEq)]
pub enum AppError {
    TaskNotFound(u32),
    EmptyStorage,
}

pub struct App {
    storage: Box<dyn Storage<Error = StorageError>>,
}

impl App {
    pub fn new(storage: Box<dyn Storage<Error = StorageError>>) -> Self {
        App { storage }
    }

    pub fn dispatch(&self, _command: Commands) -> Result<(), AppError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{any::Any, cell::RefCell};

    use super::*;
    use task_cli::core::tasks::{Status, Task};
    use task_cli::storage::StorageError;

    pub struct MockStorage {
        tasks: Vec<Task>,
        list_called_times: RefCell<u8>,
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

        fn update(&mut self, id: u32, description: String) -> Result<(), Self::Error> {
            if self.tasks.is_empty() {
                return Err(StorageError::EmptyStorage);
            }
            let task = self.tasks.iter_mut().find(|t| t.id == id);
            match task {
                Some(t) => {
                    t.description = description;
                    Ok(())
                }
                None => Err(StorageError::TaskNotFound(id)),
            }
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
                list_called_times: RefCell::new(0),
            }
        }
    }

    #[test]
    fn app_dispatch_creates_one_task() {
        let app = App::new(Box::new(MockStorage::new()));
        let command = Commands::Create("Task".into());
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
        let app = App::new(Box::new(MockStorage::new()));
        for i in 1..=3 {
            let _result = app.dispatch(Commands::Create(format!("Task {}", i)));
        }
        for (i, task) in app.storage.list().iter().enumerate() {
            assert_eq!(i as u32, task.id);
        }
    }

    #[test]
    fn app_dispatch_removes_one_task() {
        let app = App::new(Box::new(MockStorage::new()));
        let _result = app.dispatch(Commands::Create("Remove me".into()));
        let command = Commands::Remove(1);
        let result = app.dispatch(command);
        assert!(result.is_ok());
        assert_eq!(app.storage.list(), vec![]);
    }

    #[test]
    fn app_dispatch_trying_to_remove_from_empty_storage_returns_empty_storage_error() {
        let app = App::new(Box::new(MockStorage::new()));
        let command = Commands::Remove(1);
        let result = app.dispatch(command);
        assert!(result.is_err());
        assert_eq!(result, Err(AppError::EmptyStorage));
    }

    #[test]
    fn app_dispatch_trying_to_remove_invalid_id_returns_task_not_found_error() {
        let app = App::new(Box::new(MockStorage::new()));
        let _result = app.dispatch(Commands::Create("Task".into()));
        let command = Commands::Remove(2);
        let result = app.dispatch(command);
        assert!(result.is_err());
        assert_eq!(result, Err(AppError::TaskNotFound(2)));
    }

    #[test]
    fn app_dispatch_updates_one_task() {
        let app = App::new(Box::new(MockStorage::new()));
        let _result = app.dispatch(Commands::Create("Rename me".into()));
        let command = Commands::Update(1, "Renamed".into());
        let result = app.dispatch(command);
        assert!(result.is_ok());
        assert_eq!(
            app.storage.list(),
            vec![Task {
                id: 1,
                description: "Renamed".into(),
                status: Status::Todo,
            }]
        )
    }

    #[test]
    fn app_dispatch_trying_to_update_from_empty_storage_returns_empty_storage_error() {
        let app = App::new(Box::new(MockStorage::new()));
        let command = Commands::Update(1, "Empty Storage".into());
        let result = app.dispatch(command);
        assert!(result.is_err());
        assert_eq!(result, Err(AppError::EmptyStorage));
    }

    #[test]
    fn app_dispatch_trying_to_update_invalid_id_returns_task_not_found_error() {
        let app = App::new(Box::new(MockStorage::new()));
        let _result = app.dispatch(Commands::Create("Task".into()));
        let command = Commands::Update(2, "Not Found".into());
        let result = app.dispatch(command);
        assert!(result.is_err());
        assert_eq!(result, Err(AppError::TaskNotFound(2)));
    }

    #[test]
    fn app_dispatch_calls_storage_list_method() {
        let app = App::new(Box::new(MockStorage::new()));
        let list_called_times = &get_mock_storage(&app).list_called_times;
        assert_eq!(*list_called_times.borrow(), 0);
        let result = app.dispatch(Commands::List);
        let list_called_times = &get_mock_storage(&app).list_called_times;
        assert!(result.is_ok());
        assert_eq!(*list_called_times.borrow(), 1);
    }

    fn get_mock_storage(app: &App) -> &MockStorage {
        (&app.storage as &dyn Any)
            .downcast_ref::<MockStorage>()
            .unwrap()
    }
}
