mod worker;

use self::worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// ## Panics
    ///
    /// - When size == 0.
    pub fn new(size: usize) -> Self {
        assert!(size != 0);
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id));
        }
        ThreadPool { workers }
    }

    pub fn execute<F>(&self, _: F)
    where F: FnOnce() + Send + 'static {
        //
    }
}
