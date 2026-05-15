// todo_cli/src/models/todo.rs
pub struct Task {
    pub id: u32,
    pub description: String,
    pub title: String,
    pub completed: bool,
}

#[derive(Default)]
pub struct TodoList {
    pub tasks: Vec<Task>,
    pub next_id: u32,
}

