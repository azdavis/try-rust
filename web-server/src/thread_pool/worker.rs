use std::thread;

pub struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new worker.
    pub fn new(id: usize) -> Self {
        let handle = thread::spawn(|| ());
        Worker { id, handle }
    }
}
