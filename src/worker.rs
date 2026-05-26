use crate::buffer::{Buffer, BufferTrait};
use std::{
    sync::{atomic::AtomicUsize, atomic::Ordering, Arc, Condvar, Mutex},
    thread,
};

pub struct Worker {}

impl Worker {
    pub fn new() -> Self {
        return Worker {};
    }
    pub fn start(
        &self,
        buffer: Arc<(Mutex<Buffer>, Condvar)>,
        active_threads: Arc<AtomicUsize>,
        tasks_executed: Arc<AtomicUsize>,
        tasks_pending: Arc<AtomicUsize>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            active_threads.fetch_add(1, Ordering::Relaxed);

            loop {
                let task = {
                    let (lock, condvar) = &*buffer;
                    let mut buffer = lock.lock().unwrap();
                    condvar.notify_all();
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
                        let (lock, condvar) = &*buffer;
                        let guard = lock.lock().unwrap();
                        let _guard = condvar.wait(guard).unwrap();
                    }
                }
            }
            // ESTA COSA NO SE EJECUTA, TOCA METER ALGUN SHUTDOWN SIGNAL PARA QUE HAGA ALGO
            active_threads.fetch_sub(1, Ordering::Relaxed);
        })
    }
}
