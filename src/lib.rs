use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// defined a 'boxed' closure (a pointer to a function, stored on the heap)
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// create a new ThreadPool.
    ///
    /// size is the number of threads in pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        // check threadpool isn't of size 0
        assert!(size > 0);

        // create the multiple producer, single consumer channel
        let (sender, receiver) = mpsc::channel();

        // atomic smart pointer to a mutex locking the job queue
        let receiver = Arc::new(Mutex::new(receiver));

        // vector to hold the workers
        let mut workers = Vec::with_capacity(size);

        // create threads and store them in the vector, clone the pointer
        // to the receiver to give the new Worker reference to it
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // Return the newely created threadpool
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        // the closure passed should implement FnOnce, Send, and have a static lifetime
        //  - FnOnce, closure should have a trait that it can only run once
        //  - Send, it should implement Send, being able to be passed between threads
        //  - 'static, lifetime of all data in the closure should last as long as the program
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // grab the mutex, unwrap the error, receive from the FIFO, unwrap that too
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            // execute the job
            job();
        });

        Worker { id, thread }
    }
}
