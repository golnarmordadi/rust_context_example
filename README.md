# Context Example

This Rust project demonstrates two key concepts:

*Thread Context:* Creating and managing threads to perform concurrent operations.
*Process Context:* Spawning and interacting with separate system processes.

## Features

Thread Context: Spawns multiple threads to perform simple concurrent tasks and waits for all threads to complete.

Process Context: Executes system commands in a separate process and handles platform-specific differences (Windows vs. Unix-like systems).

## Prerequisites

Rust and Cargo installed. If you don't have them installed, follow the installation guide at rust-lang.org.

## Build the Project

```bash cargo build```

## Run the Project

```bash cargo run```

## Platform Support

- Windows: Uses cmd /C dir to list directory contents.
- Unix-like Systems (Linux, macOS): Uses ls -la to list directory contents.

## Result Unix-like Systems - Linux

```sh
Starting Thread Context Example...

Thread 0 is running.
Thread 0 finished: Result = 0
Thread 1 is running.
Thread 1 finished: Result = 1
Thread 3 is running.
Thread 3 finished: Result = 9
Thread 4 is running.
Thread 4 finished: Result = 16
Thread 2 is running.
Thread 2 finished: Result = 4

Thread Context Example Completed.

Starting Process Context Example...

total 32
drwxrwxr-x 5 golnar golnar 4096 Aug 20 01:41 .
drwxrwxr-x 4 golnar golnar 4096 Aug 20 01:38 ..
-rw-rw-r-- 1 golnar golnar  159 Aug 20 01:41 Cargo.lock
-rw-rw-r-- 1 golnar golnar   86 Aug 20 01:40 Cargo.toml
drwxrwxr-x 8 golnar golnar 4096 Aug 20 01:39 .git
-rw-rw-r-- 1 golnar golnar   22 Aug 20 01:38 README.md
drwxrwxr-x 2 golnar golnar 4096 Aug 20 01:39 src
drwxrwxr-x 3 golnar golnar 4096 Aug 20 01:41 target
Process exited with status: exit status: 0

Process Context Example Completed.
```
