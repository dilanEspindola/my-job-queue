use job_queue_rust::{
    buffer::{Buffer, BufferTrait},
    utils::print_status_box,
    worker::Worker,
};
use std::{
    io::Read,
    os::unix::net::{UnixListener, UnixStream},
};
use std::{
    io::{self},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};

fn main() -> std::io::Result<()> {
    let socket_path = "test.sock";

    let _ = std::fs::remove_file(socket_path);

    let listener = UnixListener::bind(socket_path)?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut socket_stream) => {
                let mut buf = [0; 1024];

                let n = socket_stream.read(&mut buf)?;
                let received_data = String::from_utf8_lossy(&buf[..n]);
                println!("Received data: {}", received_data);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    // let total_tasks = 30;
    // let active_threads = Arc::new(AtomicUsize::new(0));
    // let tasks_executed = Arc::new(AtomicUsize::new(0));
    // let tasks_pending = Arc::new(AtomicUsize::new(total_tasks));

    // let _buffer = Arc::new((Mutex::new(Buffer::new(Some(100))), Condvar::new()));

    // let (buffer, cvar) = &*_buffer;

    // let mut handles = vec![];

    // let worker1 = Worker::new();
    // let handle1 = worker1.start(
    //     Arc::clone(&_buffer),
    //     Arc::clone(&active_threads),
    //     Arc::clone(&tasks_executed),
    //     Arc::clone(&tasks_pending),
    // );

    // let worker2 = Worker::new();
    // let handle2 = worker2.start(
    //     Arc::clone(&_buffer),
    //     Arc::clone(&active_threads),
    //     Arc::clone(&tasks_executed),
    //     Arc::clone(&tasks_pending),
    // );

    // let worker3 = Worker::new();
    // let handle3 = worker3.start(
    //     Arc::clone(&_buffer),
    //     Arc::clone(&active_threads),
    //     Arc::clone(&tasks_executed),
    //     Arc::clone(&tasks_pending),
    // );
    // let worker4 = Worker::new();
    // let handle4 = worker4.start(
    //     Arc::clone(&_buffer),
    //     Arc::clone(&active_threads),
    //     Arc::clone(&tasks_executed),
    //     Arc::clone(&tasks_pending),
    // );

    // println!("waiting for tasks to finish...");
    // thread::sleep(Duration::from_secs(2));

    // for i in 0..total_tasks {
    //     let id = format!("task {}", i + 1);
    //     let id_clone = id.clone();
    //     match buffer.lock().unwrap().add(
    //         String::from(id),
    //         cvar,
    //         Box::new(move || {
    //             thread::sleep(Duration::from_secs(1));
    //             println!("{}  executed", id_clone);
    //         }),
    //     ) {
    //         Ok(id) => println!("Task {} added to the queue", id),
    //         Err(e) => eprintln!("Error adding task: {}", e),
    //     }
    // }

    // // buffer
    // //     .lock()
    // //     .unwrap()
    // //     .add(
    // //         String::from("task 1"),
    // //         Box::new(|| {
    // //             println!("Task 1 executed");
    // //         }),
    // //     )
    // //     .unwrap_or("Error adding task".to_string());

    // handles.push(handle1);
    // handles.push(handle2);
    // handles.push(handle3);
    // handles.push(handle4);

    // for handle in handles {
    //     match handle.join() {
    //         Ok(_) => (),
    //         Err(e) => eprintln!("Error joining thread: {:?}", e),
    //     }
    // }

    // print_status_box(
    //     active_threads.load(Ordering::Acquire),
    //     tasks_executed.load(Ordering::Acquire),
    //     tasks_pending.load(Ordering::Acquire),
    // );

    Ok(())
}
