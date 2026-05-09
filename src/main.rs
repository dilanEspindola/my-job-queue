use job_queue_rust::{
    buffer::{Buffer, BufferTrait},
    worker::{self, Worker},
};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let buffer = Arc::new(Mutex::new(Buffer::new(Some(100))));

    match buffer.lock().unwrap().add(String::from("task 1"), || {
        thread::sleep(Duration::from_secs(3));
        println!("Task 1 executed");
    }) {
        Ok(id) => println!("Task {} added successfully", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    match buffer.lock().unwrap().add(String::from("task 2"), || {
        thread::sleep(Duration::from_secs(1));
        println!("Task 2 executed");
    }) {
        Ok(id) => println!("Task {} added successfully", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    match buffer.lock().unwrap().add(String::from("task 3"), || {
        thread::sleep(Duration::from_secs(5));
        println!("Task 3 executed");
    }) {
        Ok(id) => println!("Task {} added successfully", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    match buffer.lock().unwrap().add(String::from("task 4"), || {
        thread::sleep(Duration::from_secs(4));
        println!("Task 4 executed");
    }) {
        Ok(id) => println!("Task {} added successfully", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    println!("\n");

    let mut handles = vec![];

    let worker1 = Worker::new();
    let handle1 = worker1.start(Arc::clone(&buffer));

    let worker2 = Worker::new();
    let handle2 = worker2.start(Arc::clone(&buffer));

    let worker3 = Worker::new();
    let handle3 = worker3.start(Arc::clone(&buffer));

    handles.push(handle1);
    handles.push(handle3);
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }
}
