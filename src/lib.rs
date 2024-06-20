use std::{io::{Read, Write}, net::TcpStream, sync::{mpsc, Arc, Mutex}, thread::{spawn, JoinHandle, ThreadId}};

use channel_manager::SharedChannelManager;
use protocol::Protocol;


pub mod conn;
pub mod models;
pub mod schema;
pub mod protocol;
pub mod dto;
pub mod channel_manager;


pub fn handle_connection(mut stream: TcpStream, channel_manager: SharedChannelManager) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = Protocol::handle_request(&buffer);

    // define o canal para ser enviado
    let channel = response.header.channel.clone();

    // Inscreve o request no canal
    {
        let mut manager = channel_manager.lock().unwrap();
        manager.subscribe(channel, stream.try_clone().unwrap());

        if response.req_type == protocol::RequestType::Send {
            manager.send_to_channel(&response.header.channel.clone(), &response.to_string());
        }else{
            stream.write_all(response.to_string().as_bytes()).unwrap();
        }
    }

    // manda a resposta de volta para o cliente
    stream.flush().unwrap();
}

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
                Err(error) => {

                    println!("{}", error.to_string());
                    println!("Worker disconnected; shutting down.");
                    //break;
                } 
            }

        });
        let id: ThreadId = thread.thread().id();

        Worker{id, thread: Some(thread)}
    }
}