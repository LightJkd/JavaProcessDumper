# JavaProcessDumper

JavaProcessDumper is a command-line tool to dump the class histogram of a running Java process. It uses the `jmap` utility to generate a histogram of the Java heap memory for a specified process ID (PID) and saves the output to a file.

## Features
- Prompt the user to enter the PID of a Java process
- Generate a class histogram of the specified Java process
- Save the class histogram to a file

## Installation and Usage

### Prerequisites
- Rust programming language installed. You can install Rust from [here](https://www.rust-lang.org/tools/install).
- `jmap` utility should be available on your system. It is part of the JDK.

### Windows

1. **Install Rust:**
   Download and install Rust from the official website: [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Build the Program:**
   Open a terminal and run the following commands:
```rust
   git clone https://github.com/LightJkd/JavaProcessDumper.git
   cd JavaProcessDumper
   cargo build --release

```

3.  **Run the Program:**
```
target\release\JavaProcessDumper.exe

```


### MacOS
1. **Install Rust:**
Open a terminal and install Rust using Homebrew:
```rust
brew install rustup
rustup-init

```

2. **Build the Program:**
Clone the repository and build the program:
```rust
git clone https://github.com/LightJkd/JavaProcessDumper.git
cd JavaProcessDumper
cargo build --release

```
3.  **Run the Program:**
```
./target/release/JavaProcessDumper

```

### Linux

1. **Install Rust:**
Open a terminal and install Rust:
```rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

```

2. **Build the Program:**
Clone the repository and build the program:
```rust
git clone https://github.com/LightJkd/JavaProcessDumper.git
cd JavaProcessDumper
cargo build --release

```

3.  **Run the Program:**
```
./target/release/JavaProcessDumper

```


### How It Works
The program prompts the user to enter the PID of a Java process. Using the jmap utility, it generates a class histogram of the Java heap memory and saves it to a file.

**Code Snippet**
```rust
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

``` 

“Important Notes*
 * The `jmap` utility must be available on your system and accessible from the command line.”
 * The program currently supports dumping the class histogram for Java processes only.”

 # License:
 * This project is licensed under the MIT License. 
