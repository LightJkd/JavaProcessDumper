use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::env;

fn main() {
    println!("Welcome!");

    loop {
        println!("Choose an action:");
        println!("1. Enter the PID of the process");
        println!("2. Instructions on how to find the PID");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                let pid = get_pid_from_user();
                if let Some(pid) = pid {
                    dump_classes(pid);
                    break;
                }
            }
            2 => println!("Instructions: \n1. Open the console\n2. Enter the command \"top\" (it will display all processes, but we are looking for a Java process that consumes the most RAM)\n3. Find the PID of the Java process in the leftmost column under the PID heading, then copy it."),
            _ => println!("Invalid choice, try again"),
        }
    }

    println!("Press any key to exit...");
    let _ = io::stdin().read_line(&mut String::new());
}

fn get_pid_from_user() -> Option<u32> {
    print!("Enter the PID of the process: ");
    io::stdout().flush().unwrap();

    let mut pid = String::new();
    io::stdin().read_line(&mut pid).expect("Failed to read line");

    pid.trim().parse().ok()
}

fn dump_classes(pid: u32) {
    let username = whoami::username();
    let dump_file_path = format!("/home/{}/dump_java.txt", username);

    let output = Command::new("jmap")
        .arg("-histo")
        .arg(pid.to_string())
        .output()
        .expect("Failed to execute jmap command");

    if output.status.success() {
        let histo_output = String::from_utf8_lossy(&output.stdout);
        let mut file = File::create(&dump_file_path).expect("Failed to create file");
        writeln!(file, "{}", histo_output).expect("Failed to write to file");

        println!("Class histogram successfully saved to dump_java.txt");
        let path = PathBuf::from(&dump_file_path);
        println!("File available at: {:?}", path);
    } else {
        println!("Failed to dump Java process memory");
    }
}
