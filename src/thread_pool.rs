use logger::log;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread_builder = thread::Builder::new();
        match thread_builder.spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    log!(debug "worker {} got a job", id);
                    job();
                }
                Err(_) => {
                    log!(debug "worker {} disconnected", id);
                    break;
                }
            }
        }) {
            Ok(thread) => Worker {
                id,
                thread: Some(thread),
            },
            Err(err) => panic!("unable to create thread, {:?}", err),
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers: Vec<_> = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        if let Err(err) = self.sender.as_ref().unwrap().send(job) {
            log!(error "error sending job: {:?}", err);
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            log!(debug "shutting down worker {}", (worker.id));

            if let Some(thread) = worker.thread.take() {
                if let Err(err) = thread.join() {
                    log!(error "error shutting down thread {}: {:?}", (worker.id), err);
                }
            }
        }
    }
}
