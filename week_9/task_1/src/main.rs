use std::thread::{self, JoinHandle};

fn main() {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..3 {
        let handle = thread::spawn(move || {
            println!("Thread {i} is running");
        });
        handles.push(handle);
    }
}