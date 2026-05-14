use job_queue_rust::{
    buffer::{Buffer, BufferTrait},
    worker::Worker,
};
use std::{
    io::Error,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

fn running_background() {
    thread::sleep(Duration::from_secs(2));
    println!("Task 4 executed");
}

fn main() {
    let active_threads = Arc::new(AtomicUsize::new(0));
    let tasks_executed = Arc::new(AtomicUsize::new(0));
    let tasks_pending = Arc::new(AtomicUsize::new(4));
    let buffer = Arc::new(Mutex::new(Buffer::new(Some(100))));

    match buffer.lock().unwrap().add(String::from("task 1"), || {
        thread::sleep(Duration::from_secs(3));
        println!("Task 1 executed");
    }) {
        Ok(id) => println!("Task {} added to the queue", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    match buffer.lock().unwrap().add(String::from("task 2"), || {
        thread::sleep(Duration::from_secs(1));
        println!("Task 2 executed");
    }) {
        Ok(id) => println!("Task {} added to the queue", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    match buffer.lock().unwrap().add(String::from("task 3"), || {
        thread::sleep(Duration::from_secs(5));
        println!("Task 3 executed");
    }) {
        Ok(id) => println!("Task {} added to the queue", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    match buffer
        .lock()
        .unwrap()
        .add(String::from("task 4"), running_background)
    {
        Ok(id) => println!("Task {} added to the queue", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    println!("\n");

    let mut handles = vec![];

    let worker1 = Worker::new();
    let handle1 = worker1.start(
        Arc::clone(&buffer),
        Arc::clone(&active_threads),
        Arc::clone(&tasks_executed),
        Arc::clone(&tasks_pending),
    );

    let worker2 = Worker::new();
    let handle2 = worker2.start(
        Arc::clone(&buffer),
        Arc::clone(&active_threads),
        Arc::clone(&tasks_executed),
        Arc::clone(&tasks_pending),
    );

    handles.push(handle1);
    // handles.push(handle3);
    handles.push(handle2);
    // handles.push(handle4);

    for handle in handles {
        handle.join().unwrap();
    }

    print_status_box(
        active_threads.load(Ordering::Acquire),
        tasks_executed.load(Ordering::Acquire),
        tasks_pending.load(Ordering::Acquire),
    );
}

fn print_status_box(active_threads: usize, tasks_executed: usize, tasks_pending: usize) {
    let width = 40;
    let border = format!("+{}+", "-".repeat(width));

    let row = |label: &str, value: usize| format!("| {:<28} {:>8} |", label, value);

    println!("{}", border);
    println!("| {:<38} |", "Job Queue Status");
    println!("{}", border);
    println!("{}", row("Active threads:", active_threads));
    println!("{}", row("Tasks executed:", tasks_executed));
    println!("{}", row("Tasks pending:", tasks_pending));
    println!("{}", border);
}
