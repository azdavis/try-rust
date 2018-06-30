use std::sync::mpsc;

mod worker;

use self::worker::Worker;

pub struct Job;

pub struct ThreadPool {
    tx: mpsc::Sender<Job>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool with size many workers. Panic if size == 0.
    pub fn new(size: usize) -> Self {
        assert!(size != 0);
        let (tx, rx) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, rx));
        }
        ThreadPool { tx, workers }
    }

    /// Register f for later execution. As soon as at least one of the threads
    /// in the pool is not busy, exactly one of them will execute f.
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static {
        //
    }
}
