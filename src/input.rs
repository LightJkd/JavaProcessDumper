use std::io::{self, Write};

pub fn read_user_choice() -> u32 {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    choice.trim().parse().unwrap_or(0)
}

pub fn get_pid_from_user() -> Option<u32> {
    print!("Enter the PID of the process: ");
    io::stdout().flush().unwrap();

    let mut pid = String::new();
    io::stdin().read_line(&mut pid).expect("Failed to read line");

    pid.trim().parse().ok()
}
