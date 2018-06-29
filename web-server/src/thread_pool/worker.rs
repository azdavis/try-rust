pub struct Worker {
    id: usize,
}

impl Worker {
    /// Create a new worker.
    pub fn new(id: usize) -> Self {
        Worker { id }
    }
}
