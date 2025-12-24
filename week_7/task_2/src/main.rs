use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // 1. Run the command and capture its stdout
    let output = Command::new("echo")
        .arg("Rust Process Management")
        .output() // This executes the command and waits for it to finish
        .expect("Failed to execute command");

    // 2. Create the output.txt file
    let mut file = File::create("output.txt")?;

    // 3. Write the captured stdout (as bytes) to the file
    file.write_all(&output.stdout)?;

    println!("Successfully captured output to output.txt");

    Ok(())
}