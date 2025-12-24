use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Run the command and capture its stdout
    let output = Command::new("echo")
        .arg("Rust Process Management")
        .output() // Waits for the process to finish and returns its output
        .expect("Failed to execute echo");

    // Create the file and write the captured bytes to it
    let mut file = File::create("output.txt")?;
    file.write_all(&output.stdout)?;

    println!("Output successfully written to output.txt");
    Ok(())
}