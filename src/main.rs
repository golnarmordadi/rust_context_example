use std::thread;
use std::process::Command;

fn main() {
    println!("Rust Project for Thread Context and Process Context\n");

    // 1. Thread Context Example
    thread_context_example();

    // 2. Process Context Example
    process_context_example();
}

// Function demonstrating thread context
fn thread_context_example() {
    println!("Starting Thread Context Example...\n");

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread {} is running.", i);

            // Simulate some work in the thread
            let result = i * i;
            println!("Thread {} finished: Result = {}", i, result);
        });

        // Push the handle to the vector to join later
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nThread Context Example Completed.\n");
}

// Function demonstrating process context
fn process_context_example() {
    println!("Starting Process Context Example...\n");

    // Spawn a separate process based on the platform
    let status = if cfg!(target_os = "windows") {
        // Windows platform
        let output = Command::new("cmd")
            .args(&["/C", "dir"])
            .output()
            .expect("Failed to execute process");
        
        // Print output from the process
        print!("{}", String::from_utf8_lossy(&output.stdout));
        
        output.status
    } else {
        // Unix-like platform
        let output = Command::new("ls")
            .arg("-la")
            .output()
            .expect("Failed to execute process");
        
        // Print output from the process
        print!("{}", String::from_utf8_lossy(&output.stdout));
        
        output.status
    };

    println!("Process exited with status: {}\n", status);

    println!("Process Context Example Completed.\n");
}
