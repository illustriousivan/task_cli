use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Status,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}
