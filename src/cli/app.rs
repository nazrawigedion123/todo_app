// todo_app/src/cli/app.rs
use super::handlers::{handle_add, handle_list, handle_mark_as_complete};

use std::io::{self, Write};

use crate::repository::{TodoRepository, create_repository};

pub struct App {
    repo: Box<dyn TodoRepository>,
}

impl App {
    pub fn new() -> Self {
        Self {
            repo: (create_repository()),
        }
    }
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // let mut todo = TodoList::new();
        print!("Wellcome to a todo list================================");
        loop {
            print!("\n>");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input {
                "quit" | "exit" => {
                    println!("Goodbye!");
                    break;
                }
                "list" => {
                    handle_list(&*self.repo);
                }
                "add" => {
                    handle_add(&mut *self.repo);
                }
                "complete task" | "complete" => {
                    handle_mark_as_complete(&mut *self.repo);
                }

                _ => println!("unknown command"),
            }
        }
        Ok(())
    }
}
