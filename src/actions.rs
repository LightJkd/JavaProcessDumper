use std::process::{Command, Stdio};
use std::fs::File;
use std::path::PathBuf;
use std::env;
use std::io::{self, Write};

pub fn dump_classes(pid: u32) {
    let dump_file_path = get_dump_file_path();

    match Command::new("jmap")
        .arg("-histo")
        .arg(pid.to_string())
        .stdout(Stdio::piped())
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                save_histogram_to_file(&dump_file_path, &output.stdout);
                println!("Class histogram successfully saved to dump_java.txt");
                let path = PathBuf::from(&dump_file_path);
                println!("File available at: {:?}", path);
            } else {
                eprintln!("Java arguments detected: -XX:+DisableAttachMechanism\nBAN this user!");
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

fn get_dump_file_path() -> String {
    match env::consts::OS {
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
    }
}

fn save_histogram_to_file(dump_file_path: &str, histo_output: &[u8]) {
    let histo_output_str = String::from_utf8_lossy(histo_output);
    let mut file = File::create(dump_file_path).expect("Failed to create file");
    writeln!(file, "{}", histo_output_str).expect("Failed to write to file");
}
