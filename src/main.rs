
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use small_talk::channel_manager::ChannelManager;
use small_talk::handle_connection;
use small_talk::ThreadPool;


fn main() {
    // escutando a porta 6969
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    let pool = ThreadPool::new(40);

    // canal para gerenciar os channels
    let channel_manager = Arc::new(Mutex::new(ChannelManager::new()));

    // fazendo um laço a partir da stream de dados vinda do listener
    // for stream in listener.incoming().take(2) {
    //     let stream = stream.unwrap();
    //     let channel_manager = Arc::clone(&channel_manager);

    //     pool.execute(move || {
    //         handle_connection(stream, channel_manager)
    //     });
    // }

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let channel_manager = Arc::clone(&channel_manager);

        thread::spawn(|| {
            println!("Connection established");
            handle_connection(stream, channel_manager);
        });
    }
}
