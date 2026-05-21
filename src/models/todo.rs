// todo_cli/src/models/todo.rs

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub title: String,
    pub completed: bool,
}

#[derive(Default)]
pub struct TodoList {
    pub tasks: Vec<Task>,
    pub next_id: usize,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TodoError {
    #[error("Title canot be empty")]
    EmptyTitle,
    #[error("Description cannot be empty")]
    EmptyDescription,
    #[error("Invalid task index: {0}")]
    InvalidIndex(usize),
    #[error("task alreay complete ")]
    TaskAlreadyComplete(usize),
    #[error("todo list already empty")]
    AlreadyEmpty,
    #[error("database err")]
    DatabaseError(String),
}
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "✓" } else { "O" };
        write!(f, "[{}] {}-{}", status, self.title, self.description)
    }
}
