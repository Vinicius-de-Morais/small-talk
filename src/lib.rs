use std::{os::unix::thread, sync::{mpsc::{self, Receiver}, Arc, Mutex}, thread::{spawn, JoinHandle, ThreadId}};

pub mod conn;
pub mod models;
pub mod schema;
pub mod protocol;

// estrutura responsável por inicializar uma thread de estruturas
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);
        
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        // criando um novo tipo para o job ser mutável
        let receiver = Arc::new(Mutex::new(receiver));

        for _ in 0..size{
            workers.push(Worker::new(Arc::clone(&receiver)));
        }
        
        ThreadPool {workers, sender: Some(sender)}
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static 
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();

    }
}

// implementando mecanismo para "desligar" os workers e threads
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers{
            println!("Shuttingdown worker {:?}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


// workers que irão inicializar as threads
pub struct Worker {
    id: ThreadId,
    thread: Option<JoinHandle<()>>
}

impl Worker {
    pub fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        
        // o receiver nesse caso é uma função para ser executada.
        let thread = spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker got a job; executing.");
        
                    job();
                }
                Err(_) => {
                    println!("Worker disconnected; shutting down.");
                    break;
                } 
            }

        });
        let id: ThreadId = thread.thread().id();

        Worker{id, thread: Some(thread)}
    }
}