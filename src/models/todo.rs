// todo_cli/src/models/todo.rs

use std::fmt;

#[derive(Clone)]
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

#[derive(Debug, thiserror::Error)]
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

// impl TodoList {
//     pub fn new() -> Self {
//         TodoList {
//             tasks: Vec::new(),
//             next_id: 1,
//         }
//     }

//     pub fn add_task(&mut self, title: String, description: String) -> Result<(), TodoError> {
//         if title.trim().is_empty() {
//             return Err(TodoError::EmptyTitle);
//         }
//         if description.trim().is_empty() {
//             return Err(TodoError::EmptyDescription);
//         }

//         let task = Task {
//             id: self.next_id,
//             title,
//             description,
//             completed: false,
//         };
//         self.tasks.push(task);
//         self.next_id += 1;
//         // println!("task added");
//         Ok(())
//     }
//     pub fn list_tasks(&self) -> Vec<&Task> {
//         self.tasks.iter().collect()
//     }
//     pub fn get_task(&mut self, index: usize) -> Option<&Task> {
//         self.tasks.get(index)
//     }
//     pub fn remove_task(&mut self, index: usize) -> Option<Task> {
//         if index < self.tasks.len() {
//             Some(self.tasks.remove(index))
//         } else {
//             None
//         }
//     }
//     pub fn complete_task(&mut self, index: usize) -> Result<(), TodoError> {
//         if let Some(task) = self.tasks.get_mut(index) {
//             task.completed = true;
//             Ok(())
//         } else {
//             Err(TodoError::InvalidIndex(index))
//         }
//     }
//     pub fn task_count(&self) -> usize {
//         self.tasks.len()
//     }
//     pub fn is_empty(&self) -> bool {
//         self.tasks.is_empty()
//     }
// }
