// todo_cli/src/cmd/main.rs
use std::io::{self, Write};
use todo_app::{TodoList};

fn main() {
    let mut todo = TodoList::new();
    print!("Wellcome to a todo list================================");
    loop {
        print!("\n>");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "quit" | "exit" => break,
            "list" => todo.list_tasks(),
            "add" => {
                let mut title = String::new();
                loop {
                    print!("\nEnter title: ");
                    io::stdout().flush().unwrap();

                    io::stdin().read_line(&mut title).unwrap();
                    let title = title.trim();
                    if !title.is_empty() {
                        break;
                    } else {
                        print!("\ntitle cant be empty please enter title again")
                    }
                }
                let mut description = String::new();
                loop {
                    print!("Enter description: ");
                    io::stdout().flush().unwrap();

                    io::stdin().read_line(&mut description).unwrap();
                    let description = description.trim();
                    if !description.is_empty() {
                        break;
                    } else {
                        println!("description cant be empty please enter description again")
                    }
                }

                todo.add_task(title, description);
            }
            _ => println!("unknown command"),
        }
    }
}
