use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

use super::Job;

pub struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new worker.
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || {
            let f = rx.lock().unwrap().recv().unwrap();
            println!("id {} got a job", id);
            f.call_box();
        });
        Worker { id, handle }
    }
}
