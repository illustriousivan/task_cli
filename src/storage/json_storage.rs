use std::path::PathBuf;

use super::{Storage, StorageError};
use crate::core::tasks::{Status, Task};

pub struct JsonStorage {
    path: PathBuf,
}

impl JsonStorage {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    fn read_tasks(&self) -> Vec<Task> {
        match std::fs::read_to_string(&self.path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => vec![],
        }
    }

    fn write_tasks(&self, tasks: &[Task]) -> Result<(), <Self as Storage>::Error> {
        let json = serde_json::to_string_pretty(tasks).map_err(|_| StorageError::EmptyStorage)?;
        std::fs::write(&self.path, json).map_err(|_| StorageError::EmptyStorage)?;
        Ok(())
    }
}

impl Storage for JsonStorage {
    type Error = StorageError;

    fn create(&mut self, description: String) -> Result<u32, Self::Error> {
        let mut tasks = self.read_tasks();
        let next_id = if tasks.is_empty() {
            1
        } else {
            tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
        };
        tasks.push(Task {
            id: next_id,
            description,
            status: Status::Todo,
        });
        self.write_tasks(&tasks)?;
        Ok(next_id)
    }

    fn get(&self, id: u32) -> Result<Task, Self::Error> {
        let tasks = self.read_tasks();
        if tasks.is_empty() {
            return Err(StorageError::EmptyStorage);
        }
        tasks.iter().find(|t| t.id == id).cloned()
            .ok_or(StorageError::TaskNotFound(id))
    }

    fn update(&mut self, task: Task) -> Result<(), Self::Error> {
        let mut tasks = self.read_tasks();
        if tasks.is_empty() {
            return Err(StorageError::EmptyStorage);
        }
        let existing = tasks.iter_mut().find(|t| t.id == task.id);
        match existing {
            Some(t) => *t = task,
            None => return Err(StorageError::TaskNotFound(task.id)),
        }
        self.write_tasks(&tasks)?;
        Ok(())
    }

    fn remove(&mut self, id: u32) -> Result<(), Self::Error> {
        let mut tasks = self.read_tasks();
        let pos = tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or(StorageError::TaskNotFound(id))?;
        tasks.remove(pos);
        self.write_tasks(&tasks)?;
        Ok(())
    }

    fn list(&self) -> Vec<Task> {
        self.read_tasks()
    }

    fn list_by_status(&self, status: Status) -> Vec<Task> {
        self.read_tasks()
            .into_iter()
            .filter(|t| t.status == status)
            .collect()
    }

    fn clear_all(&mut self) -> Result<(), Self::Error> {
        let mut tasks = self.read_tasks();
        tasks.clear();
        self.write_tasks(&tasks)?;
        Ok(())
    }

    fn done_all(&mut self) -> Result<(), Self::Error> {
        let mut tasks = self.read_tasks();
        for task in &mut tasks {
            task.status = Status::Done;
        }
        self.write_tasks(&tasks)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // ── Helpers ──────────────────────────────────────────────

    fn setup_storage(tasks: Vec<Task>) -> (JsonStorage, NamedTempFile) {
        let mut tmp = NamedTempFile::new().unwrap();
        let json = serde_json::to_string_pretty(&tasks).unwrap();
        write!(tmp, "{json}").unwrap();
        let path = tmp.path().to_path_buf();
        let storage = JsonStorage::new(&path);
        (storage, tmp)
    }

    #[test]
    fn test_create_returns_incrementing_ids() {
        let (mut storage, _tmp) = setup_storage(vec![]);

        let id1 = storage.create("First".into()).unwrap();
        let id2 = storage.create("Second".into()).unwrap();
        let id3 = storage.create("Third".into()).unwrap();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
    }

    #[test]
    fn test_create_sets_status_to_todo() {
        let (mut storage, _tmp) = setup_storage(vec![]);

        storage.create("New task".into()).unwrap();

        let tasks = storage.list();
        assert_eq!(tasks[0].status, Status::Todo);
    }

    #[test]
    fn test_create_persists_across_reopens() {
        let (mut storage, tmp) = setup_storage(vec![]);
        let path = tmp.path().to_path_buf();

        storage.create("First".into()).unwrap();
        drop(storage);

        // Re-open from the same file — data must persist.
        let storage = JsonStorage::new(&path);
        assert_eq!(storage.list().len(), 1);
    }

    // ── get() ────────────────────────────────────────────────

    #[test]
    fn test_get_existing_task() {
        let (storage, _tmp) = setup_storage(vec![Task {
            id: 1,
            description: "Existing".into(),
            status: Status::Todo,
        }]);

        let task = storage.get(1).unwrap();
        assert_eq!(task.description, "Existing");
        assert_eq!(task.status, Status::Todo);
    }

    #[test]
    fn test_get_nonexistent_task_returns_error() {
        let (storage, _tmp) = setup_storage(vec![Task {
            id: 1,
            description: "Existing".into(),
            status: Status::Todo,
        }]);

        let result = storage.get(999);
        assert_eq!(result.unwrap_err(), StorageError::TaskNotFound(999));
    }

    #[test]
    fn test_get_empty_storage_returns_error() {
        let (storage, _tmp) = setup_storage(vec![]);

        let result = storage.get(1);
        assert_eq!(result.unwrap_err(), StorageError::EmptyStorage);
    }

    // ── update() ─────────────────────────────────────────────

    #[test]
    fn test_update_existing_task() {
        let (mut storage, _tmp) = setup_storage(vec![Task {
            id: 1,
            description: "Original".into(),
            status: Status::Todo,
        }]);

        storage.update(Task {
            id: 1,
            description: "Updated".into(),
            status: Status::Todo,
        }).unwrap();

        let tasks = storage.list();
        assert_eq!(tasks[0].description, "Updated");
    }

    #[test]
    fn test_update_nonexistent_task_returns_error() {
        let (mut storage, _tmp) = setup_storage(vec![Task {
            id: 1,
            description: "Existing".into(),
            status: Status::Todo,
        }]);

        let result = storage.update(Task {
            id: 999,
            description: "Ghost".into(),
            status: Status::Todo,
        });
        assert_eq!(result.unwrap_err(), StorageError::TaskNotFound(999));
    }

    #[test]
    fn test_update_replaces_full_task() {
        let (mut storage, _tmp) = setup_storage(vec![Task {
            id: 1,
            description: "Old".into(),
            status: Status::InProgress,
        }]);

        storage.update(Task {
            id: 1,
            description: "Changed".into(),
            status: Status::Done,
        }).unwrap();

        let tasks = storage.list();
        assert_eq!(tasks[0].description, "Changed");
        assert_eq!(tasks[0].status, Status::Done);
    }

    // ── remove() ─────────────────────────────────────────────

    #[test]
    fn test_remove_existing_task() {
        let (mut storage, _tmp) = setup_storage(vec![]);

        storage.create("Keep".into()).unwrap();
        let id = storage.create("Remove me".into()).unwrap();
        storage.remove(id).unwrap();

        assert_eq!(storage.list().len(), 1);
        assert_ne!(storage.list()[0].id, id);
    }

    #[test]
    fn test_remove_nonexistent_task_returns_error() {
        let (mut storage, _tmp) = setup_storage(vec![]);

        let result = storage.remove(999);
        assert_eq!(result.unwrap_err(), StorageError::TaskNotFound(999));
    }

    #[test]
    fn test_remove_preserves_other_tasks() {
        let (mut storage, _tmp) = setup_storage(vec![]);

        let id1 = storage.create("First".into()).unwrap();
        let id2 = storage.create("Second".into()).unwrap();
        let id3 = storage.create("Third".into()).unwrap();

        // Remove the middle one.
        storage.remove(id2).unwrap();

        let ids: Vec<u32> = storage.list().iter().map(|t| t.id).collect();
        assert_eq!(ids, vec![id1, id3]);
    }

    // ── list() ───────────────────────────────────────────────

    #[test]
    fn test_list_empty_storage() {
        let (storage, _tmp) = setup_storage(vec![]);
        assert!(storage.list().is_empty());
    }

    #[test]
    fn test_list_returns_tasks_in_order() {
        let (mut storage, _tmp) = setup_storage(vec![]);

        storage.create("A".into()).unwrap();
        storage.create("B".into()).unwrap();
        storage.create("C".into()).unwrap();

        let descs: Vec<String> = storage
            .list()
            .iter()
            .map(|t| t.description.clone())
            .collect();
        assert_eq!(descs, vec!["A", "B", "C"]);
    }

    // ── list_by_status() ─────────────────────────────────────

    #[test]
    fn test_list_by_status_filters_correctly() {
        let (storage, _tmp) = setup_storage(vec![
            Task {
                id: 1,
                description: "A".into(),
                status: Status::Todo,
            },
            Task {
                id: 2,
                description: "B".into(),
                status: Status::InProgress,
            },
            Task {
                id: 3,
                description: "C".into(),
                status: Status::Done,
            },
            Task {
                id: 4,
                description: "D".into(),
                status: Status::InProgress,
            },
        ]);

        assert_eq!(storage.list_by_status(Status::Todo).len(), 1);
        assert_eq!(storage.list_by_status(Status::InProgress).len(), 2);
        assert_eq!(storage.list_by_status(Status::Done).len(), 1);
    }

    #[test]
    fn test_list_by_status_empty_result() {
        let (mut storage, _tmp) = setup_storage(vec![]);

        storage.create("Only todo".into()).unwrap();

        let in_progress = storage.list_by_status(Status::InProgress);
        assert!(in_progress.is_empty());
    }

    // ── Integration: full lifecycle ──────────────────────────

    #[test]
    fn test_full_lifecycle() {
        let (mut storage, _tmp) = setup_storage(vec![]);

        // Create.
        let id1 = storage.create("Task one".into()).unwrap();
        let id2 = storage.create("Task two".into()).unwrap();
        assert_eq!(storage.list().len(), 2);

        // Update.
        let task = storage.get(id1).unwrap();
        let mut updated_task = task.clone();
        updated_task.description = "Updated task one".into();
        storage.update(updated_task).unwrap();
        assert_eq!(storage.list()[0].description, "Updated task one");

        // Remove.
        storage.remove(id2).unwrap();
        assert_eq!(storage.list().len(), 1);

        // Final state.
        let tasks = storage.list();
        assert_eq!(tasks[0].id, id1);
        assert_eq!(tasks[0].description, "Updated task one");
    }
}
