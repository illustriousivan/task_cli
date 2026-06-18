use crate::core::tasks::{Status, Task};

pub mod json_storage;

#[derive(Debug, Clone, PartialEq)]
pub enum StorageError {
    TaskNotFound(u32),
    EmptyStorage,
}

pub trait Storage {
    type Error: std::fmt::Debug;

    fn create(&mut self, description: String) -> Result<u32, Self::Error>;
    fn update(&mut self, id: u32, description: String) -> Result<(), Self::Error>;
    fn remove(&mut self, id: u32) -> Result<(), Self::Error>;
    fn list(&self) -> Vec<Task>;
    fn list_by_status(&self, status: Status) -> Vec<Task>;
}
