

use crate::{
    Task, TodoError,
    repository::{InMemoryRepo, sqllite::SqliteRepo},
};

use std::fmt;

//repository trait - the interface all repositories must implement

pub trait TodoRepository: Send + Sync {
    fn add_task(&mut self, title: String, description: String) -> Result<Task, TodoError>;
    fn list_tasks(&self) -> Result<Vec<Task>, TodoError>;
    fn get_task(&self, id: usize) -> Result<Option<Task>, TodoError>;
    fn remove_task(&mut self, id: usize) -> Result<Option<Task>, TodoError>;
    fn complete_task(&mut self, id: usize) -> Result<bool, TodoError>;
    fn task_count(&self) -> Result<usize, TodoError>;
    fn clear_all(&mut self) -> Result<(), TodoError>;
}

pub enum RepositoryConfig {
    InMemory,
    File { path: String },
    Sqllite { path: String },
}

impl fmt::Display for RepositoryConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryConfig::InMemory => write!(f, "InMemory"),
            RepositoryConfig::File { path } => write!(f, "File({})", path),
            RepositoryConfig::Sqllite { path } => write!(f, "SQLite({})", path),
        }
    }
}

pub fn create_repository(config: &RepositoryConfig) -> Box<dyn TodoRepository> {
    match config {
        RepositoryConfig::InMemory => Box::new(InMemoryRepo::new()),
        RepositoryConfig::File { path } => {
            Box::new(SqliteRepo::new(path).expect("failed to create sql repo"))
        }
        RepositoryConfig::Sqllite { path } => {
            Box::new(SqliteRepo::new(path).expect("failed to create sql repo"))
        }
    }
    // Box::new(InMemoryRepo::new())
}
