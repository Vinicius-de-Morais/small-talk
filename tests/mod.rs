#[cfg(test)]
mod tests {
    
    use small_talk::models::User;
    use small_talk::protocol::Protocol;
    use std::net::TcpListener;
    use std::thread;
    use std::io::{Read, Write};

    fn start_mock_server() -> (thread::JoinHandle<()>, String) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
        let addr = listener.local_addr().unwrap().to_string();
        let handle = thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        let mut buffer = [0u8; 1024];
                        let _ = stream.read(&mut buffer);
                        let _ = stream.write_all(b"Mock server response");
                    }
                    Err(e) => {
                        eprintln!("Failed to accept a connection: {:?}", e);
                    }
                }
            }
        });
        (handle, addr)
    }

    #[test]
    fn test_send() {
        // Start the mock server
        let server_addr = "127.0.0.1:6969";
        //let (server_handle, server_addr) = start_mock_server();

        // Mock user and payload
        let user = User { id: 123, nickname: "guest".to_string(), last_nickname: "".to_string(), active: true };
        let payload = json::object! {
            "command" => "/",
            "input" => "teste123",
            "channel" => "/",
        };

        // Attempt to send the request
        let result = Protocol::send(&server_addr, user, payload);

        // Check if sending was successful
        assert!(result.is_ok(), "Failed to send request: {:?}", result);

        // Clean up the server
        //drop(server_handle);
    }
}
