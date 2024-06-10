use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::fs::File;
use std::path::PathBuf;
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
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            },
        };

        match choice {
            1 => {
                let pid = get_pid_from_user();
                if let Some(pid) = pid {
                    dump_classes(pid);
                    break;
                }
            }
            2 => print_instructions(),
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
    let dump_file_path = match env::consts::OS {
        "windows" => {
            let username = env::var("USERNAME").expect("Failed to get username");
            format!("C:\\Users\\{}\\dump_java.txt", username)
        }
        "macos" => {
            let home_dir = env::var("HOME").expect("Failed to get home directory");
            format!("{}/Desktop/dump_java.txt", home_dir)
        }
        _ => {
            let username = whoami::username();
            format!("/home/{}/dump_java.txt", username)
        }
    };

    let output = Command::new("jmap")
        .arg("-histo")
        .arg(pid.to_string())
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let histo_output = String::from_utf8_lossy(&output.stdout);
                let mut file = File::create(&dump_file_path).expect("Failed to create file");
                writeln!(file, "{}", histo_output).expect("Failed to write to file");

                println!("Class histogram successfully saved to dump_java.txt");
                let path = PathBuf::from(&dump_file_path);
                println!("File available at: {:?}", path);
            } else {
                eprintln!("Failed to dump Java process memory");
            }
        }
        Err(e) => {
            eprintln!("Failed to execute jmap command: {}", e);
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Please ensure that jmap is installed and added to your PATH");
            }
        }
    }
}

fn print_instructions() {
    match env::consts::OS {
        "windows" => println!("Instructions to find PID on Windows:
1. Open Task Manager (Ctrl + Shift + Esc).
2. Go to the 'Details' tab.
3. Look for 'javaw.exe' in the 'Name' column.
4. The corresponding PID is listed in the 'PID' column."),
        "macos" => println!("Instructions to find PID on MacOS:
1. Open Terminal.
2. Enter the command `top`.
3. Look for a Java process, which will be listed as 'java' or 'javaw'.
4. The PID is listed in the leftmost column under the 'PID' heading."),
        _ => println!("Instructions:
1. Open a terminal.
2. Enter the command `top`.
3. Look for a Java process, which will be listed as 'java' or 'javaw'.
4. The PID is listed in the leftmost column under the 'PID' heading."),
    }
}
