use std::thread;

struct Worker {}

impl Worker {
    pub fn new() -> Option<Self> {
        thread::spawn(|| {
            println!("Worker thread started");
        });

        return Some(Worker {});
    }
}
