use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

use super::Msg;

pub struct Worker {
    pub id: usize,
    pub handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Create a new worker.
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Msg>>>) -> Self {
        let handle = Some(thread::spawn(move || {
            loop {
                let msg = rx.lock().unwrap().recv().unwrap();
                match msg {
                    Msg::Exec(f) => {
                        println!("id {} got a Exec", id);
                        f.call_box();
                        println!("id {} done with that Exec", id);
                    },
                    Msg::Term => {
                        println!("id {} got a Term", id);
                        break
                    },
                };
            }
        }));
        Worker { id, handle }
    }
}
