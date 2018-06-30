use std::sync::mpsc;
use std::thread;

use super::Job;

pub struct Worker {
    id: usize,
    rx: mpsc::Receiver<Job>,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new worker.
    pub fn new(id: usize, rx: mpsc::Receiver<Job>) -> Self {
        let handle = thread::spawn(|| ());
        Worker { id, rx, handle }
    }
}
