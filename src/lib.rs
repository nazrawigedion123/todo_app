pub mod models;
pub mod repository;
pub mod commands;
pub mod cli;

pub use models::todo::TodoList;
pub use models::todo::Task;

#[derive(Debug,thiserror::Error)]
pub enum TodoError{
    #[error("Title canot be empty")]
    EmptyTitle,
    #[error("Description cannot be empty")]
    EmptyDescription,
    #[error("Invalid task index: {0}")]
    InvalidIndex(usize),
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, title: String, description: String) {
        let task = Task {
            id: self.next_id,
            title,
            description,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("task added");
    }
    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            print!("no tasks yet use 'add' command to add ")
        }

        for i in &self.tasks {
            let complete = if i.completed { "✓" } else { "O" };
            println!(" {},[{}]  {} -> {}", i.id, complete, i.title, i.description)
        }
        println!()
    }
}