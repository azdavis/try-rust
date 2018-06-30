use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

use super::Job;

pub struct Worker {
    id: usize,
    rx: Arc<Mutex<mpsc::Receiver<Job>>>,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new worker.
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(|| ());
        Worker { id, rx, handle }
    }
}
