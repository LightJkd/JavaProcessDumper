mod input;
mod actions;
mod instructions;

use std::io;

fn main() {
    println!("Welcome!");

    loop {
        println!("Choose an action:");
        println!("1. Enter the PID of the process");
        println!("2. Instructions on how to find the PID");

        let choice = input::read_user_choice();
        match choice {
            1 => {
                if let Some(pid) = input::get_pid_from_user() {
                    actions::dump_classes(pid);
                    break;
                }
            }
            2 => instructions::print_instructions(),
            _ => println!("Invalid choice, try again"),
        }
    }

    println!("Press any key to exit...");
    let _ = io::stdin().read_line(&mut String::new());
}
