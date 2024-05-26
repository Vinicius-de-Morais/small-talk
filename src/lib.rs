use std::{sync::{mpsc::{self, Receiver}, Arc, Mutex}, thread::{spawn, JoinHandle, Thread, ThreadId}};

// estrutura responsável por inicializar uma thread de estruturas
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
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
        
        ThreadPool {workers, sender}
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static 
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();

    }
}

// workers que irão inicializar as threads
pub struct Worker {
    id: ThreadId,
    thread: JoinHandle<()>
}

impl Worker {
    pub fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        
        // o receiver nesse caso é uma função para ser executada.
        let thread = spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker got a job; executing.");

            job();
        });
        let id: ThreadId = thread.thread().id();

        Worker{id, thread}
    }
}