use crate::buffer::{Buffer, BufferTrait};
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub struct Worker {}

impl Worker {
    pub fn new() -> Self {
        return Worker {};
    }
    pub fn start(&self, buffer: Arc<Mutex<Buffer>>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            println!("Worker started");

            let task = {
                let mut buffer = buffer.lock().unwrap();
                buffer.remove()
            };

            match task {
                Some(task) => {
                    let item = task.item;
                    item();
                }
                None => println!("No tasks to execute"),
            }
        })
    }
}
