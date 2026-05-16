use super::input::input;
use crate::{TodoError, TodoList, cli::input::input_int};
pub fn handle_add(todo: &mut TodoList) {
    let title = input("Enter title: ");
    let description = input("Enter Description: ");
    match todo.add_task(title, description) {
        Ok(()) => println!("✓ Task added successfully!"),
        Err(TodoError::EmptyTitle) => println!("❌ {}.", TodoError::EmptyTitle),
        Err(TodoError::EmptyDescription) => println!("❌ {},", TodoError::EmptyDescription),
        Err(e) => println!("❌ Error {e}."),
    }
}

pub fn handle_list(todo: &TodoList) {
    let tasks = todo.list_tasks();
    if tasks.is_empty() {
        print!("No tasks yet! Use 'add; to create one");
        return;
    }
    println!("\nYour tasks");
    for (i, task) in tasks.iter().enumerate() {
        println!("  {},{}", i + 1, task);
    }
}

pub fn handle_mark_as_complete(todo: &mut TodoList) {
    let index= input_int("Enter index");

    match todo.complete_task(index) {
        Ok(()) => println!("Task {index} marked as completed"),
        Err(TodoError::InvalidIndex(index)) => println!("❌ {}", TodoError::InvalidIndex(index)),
        Err(e) => println!("❌ {e}"),
    }
}
