use job_queue_rust::{
    buffer::{Buffer, BufferTrait},
    worker::Worker,
};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};

fn running_background() {
    thread::sleep(Duration::from_secs(2));
    println!("Task 4 executed");
}

fn main() {
    let total_tasks = 30;
    let active_threads = Arc::new(AtomicUsize::new(0));
    let tasks_executed = Arc::new(AtomicUsize::new(0));
    let tasks_pending = Arc::new(AtomicUsize::new(total_tasks));
    let _buffer = Arc::new((Mutex::new(Buffer::new(Some(100))), Condvar::new()));

    let (buffer, _cvar) = &*_buffer;

    for i in 0..total_tasks {
        let id = format!("task {}", i + 1);
        let id_clone = id.clone();
        match buffer.lock().unwrap().add(
            String::from(id),
            Box::new(move || {
                thread::sleep(Duration::from_secs(1));
                println!("{}  executed", id_clone);
            }),
        ) {
            Ok(id) => println!("Task {} added to the queue", id),
            Err(e) => eprintln!("Error adding task: {}", e),
        }
    }

    let mut handles = vec![];

    let worker1 = Worker::new();
    let handle1 = worker1.start(
        Arc::clone(&_buffer),
        Arc::clone(&active_threads),
        Arc::clone(&tasks_executed),
        Arc::clone(&tasks_pending),
    );

    let worker2 = Worker::new();
    let handle2 = worker2.start(
        Arc::clone(&_buffer),
        Arc::clone(&active_threads),
        Arc::clone(&tasks_executed),
        Arc::clone(&tasks_pending),
    );

    let worker3 = Worker::new();
    let handle3 = worker3.start(
        Arc::clone(&_buffer),
        Arc::clone(&active_threads),
        Arc::clone(&tasks_executed),
        Arc::clone(&tasks_pending),
    );
    let worker4 = Worker::new();
    let handle4 = worker4.start(
        Arc::clone(&_buffer),
        Arc::clone(&active_threads),
        Arc::clone(&tasks_executed),
        Arc::clone(&tasks_pending),
    );

    handles.push(handle1);
    handles.push(handle2);
    handles.push(handle3);
    handles.push(handle4);

    for handle in handles {
        match handle.join() {
            Ok(_) => (),
            Err(e) => eprintln!("Error joining thread: {:?}", e),
        }
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
