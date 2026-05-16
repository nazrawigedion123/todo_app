use super::input::input;
use crate::repository::TodoRepository;
use crate::{TodoError, cli::input::input_int};
pub fn handle_add(repo: &mut dyn TodoRepository) {
    let title = input("Enter title: ");
    let description = input("Enter Description: ");
    match repo.add_task(title, description) {
        Ok(task) => println!("✓{task} Task added successfully!"),
        Err(TodoError::EmptyTitle) => println!("❌ {}.", TodoError::EmptyTitle),
        Err(TodoError::EmptyDescription) => println!("❌ {},", TodoError::EmptyDescription),
        Err(e) => println!("❌ Error {e}."),
    }
}

pub fn handle_list(repo: &dyn TodoRepository) {
    let tasks = repo.list_tasks();
    // if tasks.is_empty() {
    //     print!("No tasks yet! Use 'add; to create one");
    //     return;
    // }
    match tasks {
        Ok(tasks) => {
            println!("\nYour tasks");

            for (i, task) in tasks.iter().enumerate() {
                println!("  {},{}", i + 1, task);
            }
            if tasks.is_empty() {
                println!("no tasks yet. to add use the command 'add'");
            }
        }
        Err(e) => println!("Error {e}"),
    }
    // println!("\nYour tasks");
    // for (i, task) in tasks.iter().enumerate() {
    //     println!("  {},{}", i + 1, task);
    // }
}

pub fn handle_mark_as_complete(repo: &mut dyn TodoRepository) {
    let index = input_int("Enter index");

    match repo.complete_task(index) {
        Ok(task) => println!("Task {index}-{task} marked as completed"),
        Err(TodoError::InvalidIndex(index)) => println!("❌ {}", TodoError::InvalidIndex(index)),
        Err(e) => println!("❌ {e}"),
    }
}
