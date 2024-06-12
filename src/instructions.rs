use std::env;

pub fn print_instructions() {
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
        _ => println!("Instructions to the find PID on Linux:
1. Open a terminal.
2. Enter the command `top`.
3. Look for a Java process, which will be listed as 'java'.
4. The PID is listed in the leftmost column under the 'PID' heading."),
    }
}
