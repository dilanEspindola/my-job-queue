use job_queue_rust::buffer::{self, BufferTrait};
use std::{thread, time::Duration};

fn main() {
    let mut buffer = buffer::Buffer::new(Some(100));

    match buffer.add(String::from("task 1"), || {
        thread::sleep(Duration::from_secs(1));
        println!("Task 1 executed");
    }) {
        Ok(id) => println!("Task {} added successfully", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    match buffer.add(String::from("task 2"), || {
        thread::sleep(Duration::from_secs(2));
        println!("Task 2 executed");
    }) {
        Ok(id) => println!("Task {} added successfully", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    match buffer.add(String::from("task 2"), || {
        thread::sleep(Duration::from_secs(1));
        println!("Task 3 executed");
    }) {
        Ok(id) => println!("Task {} added successfully", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

    match buffer.add(String::from("task 4"), || {
        thread::sleep(Duration::from_secs(2));
        println!("Task 4 executed");
    }) {
        Ok(id) => println!("Task {} added successfully", id),
        Err(e) => eprintln!("Error adding task: {}", e),
    }

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
