use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;

mod worker;

use self::worker::Worker;

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct ThreadPool {
    tx: mpsc::Sender<Job>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool with size many workers. Panic if size == 0.
    pub fn new(size: usize) -> Self {
        assert!(size != 0);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, rx.clone()));
        }
        ThreadPool { tx, workers }
    }

    /// Register f for later execution. As soon as at least one of the threads
    /// in the pool is not busy, exactly one of them will execute f.
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.tx.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("id {} shutting down", worker.id);
            worker.handle.take().map(|x| x.join().unwrap());
        }
    }
}
