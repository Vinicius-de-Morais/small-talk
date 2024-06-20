#[cfg(test)]
mod tests {
    use small_talk::channel_manager::ChannelManager;
    use small_talk::models::User;
    use small_talk::protocol::Protocol;
    use small_talk::{handle_connection, ThreadPool};
    use std::io::{Read, Write};
    use std::net::Shutdown;
    use std::net::{TcpListener, TcpStream};
    use std::thread;
    use std::sync::{Arc, Mutex};

    type SharedChannelManager = Arc<Mutex<ChannelManager>>;

    // iniciar o servidor mokado
    fn start_mock_server(channel_manager: SharedChannelManager) -> (thread::JoinHandle<()>, String) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
        let addr = listener.local_addr().unwrap().to_string();
        let pool = ThreadPool::new(4);
        let handle = thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let channel_manager = channel_manager.clone();
                pool.execute(move || {
                    handle_connection(stream, channel_manager);
                });
            }
        });
        (handle, addr)
    }

    #[test]
    fn test_send() {

        //let channel_manager: SharedChannelManager = Arc::new(Mutex::new(ChannelManager::new()));

        // Começar o server e o gerenciador de canais
        //let (server_handle, server_addr) = start_mock_server(channel_manager.clone());

        // Mockar o usuario e payload
        let user = User { id: 123, nickname: "guest".to_string(), last_nickname: "".to_string(), active: true };
        let mut payload = json::JsonValue::new_object();
            payload["command"] = "/message".into();
            payload["input"] = "teste123".into();

        // Attempt to send the request
        //let result = Protocol::send(server_addr.as_str(), user, payload);
        let result = Protocol::send(user, payload).unwrap();

        // Establish TCP connection to the server
        let mut stream = TcpStream::connect("127.0.0.1:6969").expect("Failed to connect");

        stream.write_all(&result.as_bytes());

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let res = String::from_utf8_lossy(&buffer).to_string();
        println!("{:?}", res);


        let mut stream = TcpStream::connect("127.0.0.1:6969").expect("Failed to connect");
        let resultado = stream.write_all(&result.as_bytes());
        
        let a = resultado.unwrap();
        println!("{:?}", a);



        // Send the request to the server

        //assert!(result.is_ok(), "Failed to send request: {:?}", result);

        // dropa o server
        //drop(server_handle);
    }
}
