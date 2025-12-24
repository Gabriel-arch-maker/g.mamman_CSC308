use std::process::{Command, Child};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Parent process started. PID: {}", std::process::id());

    // 1. Child process running 'sleep 5'
    let mut child1 = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("failed to execute sleep");

    // 2. Child process running 'ls -la'
    let mut child2 = Command::new("ls")
        .arg("-la")
        .spawn()
        .expect("failed to execute ls");

    // 3. Child process running 'echo "Hello from child"'
    let mut child3 = Command::new("echo")
        .arg("Hello from child")
        .spawn()
        .expect("failed to execute echo");

    println!("Spawned Child 1 (sleep) PID: {}", child1.id());
    println!("Spawned Child 2 (ls) PID: {}", child2.id());
    println!("Spawned Child 3 (echo) PID: {}", child3.id());

    // Give us time to run 'pstree' or 'ps' in another terminal
    println!("Waiting 10 seconds before exiting...");
    thread::sleep(Duration::from_secs(10));

    // Cleanup
    let _ = child1.wait();
    let _ = child2.wait();
    let _ = child3.wait();
}