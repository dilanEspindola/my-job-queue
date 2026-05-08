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

    match buffer.lock().unwrap().add(
        String::from("task 1"),
        Box::new(|| {
            thread::sleep(Duration::from_secs(3));
            println!("Task 1 executed");
        }),
    ) {
        Ok(id) => println!("Task {} added successfully", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    match buffer.lock().unwrap().add(
        String::from("task 2"),
        Box::new(|| {
            thread::sleep(Duration::from_secs(2));
            println!("Task 2 executed");
        }),
    ) {
        Ok(id) => println!("Task {} added successfully", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    match buffer.lock().unwrap().add(
        String::from("task 3"),
        Box::new(|| {
            thread::sleep(Duration::from_secs(1));
            println!("Task 3 executed");
        }),
    ) {
        Ok(id) => println!("Task {} added successfully", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    match buffer.lock().unwrap().add(
        String::from("task 4"),
        Box::new(|| {
            thread::sleep(Duration::from_secs(4));
            println!("Task 4 executed");
        }),
    ) {
        Ok(id) => println!("Task {} added successfully", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    println!("\n");

    let worker = Worker::new();
    worker.start(buffer);

    // let worker2 = Worker::new();
    // worker2.start(&mut buffer);

    // let h: Vec<thread::JoinHandle<()>> = buffer
    //     .list_tasks()
    //     .iter()
    //     .map(|task| {
    //         let item = task.item;
    //         thread::spawn(move || {
    //             item();
    //         })
    //     })
    //     .collect();

    // for handle in h {
    //     handle.join().unwrap();
    // }
}
