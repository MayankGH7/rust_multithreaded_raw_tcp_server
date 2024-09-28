use std::sync::{Arc, Mutex};

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: std::sync::mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = std::sync::mpsc::channel();

    let receiver: Arc<Mutex<std::sync::mpsc::Receiver<Box<dyn FnOnce() + Send>>>> = Arc::new(Mutex::new(receiver));

    let mut workers: Vec<Worker> = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job: Box<F> = Box::new(f);
    self.sender.send(job).unwrap();
  }
}

struct Worker {
  id: usize,
  thread: std::thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>) -> Worker {
    let thread: std::thread::JoinHandle<()> = std::thread::spawn(move || loop {
      let job: Box<dyn FnOnce() + Send> = receiver.lock().unwrap().recv().unwrap();
      println!("Worker {} got a job; executing.", id);
      job();
    });
    Worker { id, thread }
  }
}
