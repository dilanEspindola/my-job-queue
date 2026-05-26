use crate::buffer::{Buffer, BufferTrait};
use std::{
    sync::{atomic::AtomicUsize, atomic::Ordering, Arc, Mutex},
    thread,
};

pub struct Worker {}

impl Worker {
    pub fn new() -> Self {
        return Worker {};
    }
    pub fn start(
        &self,
        buffer: Arc<Mutex<Buffer>>,
        active_threads: Arc<AtomicUsize>,
        tasks_executed: Arc<AtomicUsize>,
        tasks_pending: Arc<AtomicUsize>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            active_threads.fetch_add(1, Ordering::Relaxed);

            loop {
                let task = {
                    let mut buffer = buffer.lock().unwrap();
                    buffer.remove()
                };

                match task {
                    Some(task) => {
                        let item = task.item;
                        item();
                        tasks_executed.fetch_add(1, Ordering::Relaxed);
                        tasks_pending.fetch_sub(1, Ordering::Relaxed);
                    }
                    None => {
                        break;
                    }
                }
            }

            active_threads.fetch_sub(1, Ordering::Relaxed);
        })
    }
}
