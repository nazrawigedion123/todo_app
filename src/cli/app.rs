// todo_app/src/cli/app.rs
use super::handlers::{handle_add, handle_list};
use std::io::{self, Write};
use crate::TodoList;

pub fn run()->Result<(),Box<dyn std::error::Error>> {
    let mut todo = TodoList::new();
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
                handle_list(&todo);
            }
            "add" => {
                handle_add(&mut todo);
            }

            _ => println!("unknown command"),
        }
    }
    Ok(())
}
