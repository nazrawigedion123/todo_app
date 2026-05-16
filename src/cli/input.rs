use std::io;
use std::io::{Write};



pub fn input(prompt: &str) -> String {
    loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if !input.is_empty() {
            return input;
        }
        print!("input cant be empty, please try agin");
    }
}
pub fn input_int(prompt: &str) -> usize {
    loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        input=input.trim().to_string();
         if !input.is_empty() {

            match input.parse::<usize>() {
                Ok(num)=>return num,
                Err(_)=>{
                    println!("not a valid number please enter again")
                }
                
            }
           


        }
        
        
       
        print!("input cant be empty, please try agin");
    }
}
