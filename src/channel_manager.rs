use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;

pub type SharedChannelManager = Arc<Mutex<ChannelManager>>;

pub struct ChannelManager {
    channels: HashMap<String, Vec<TcpStream>>,
}

impl ChannelManager {
    pub fn new() -> Self {
        ChannelManager {
            channels: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, channel: String, stream: TcpStream) {
        self.channels.entry(channel).or_default().push(stream);
    }

    pub fn send_to_channel(&self, channel: &str, message: &str) {
        if let Some(subscribers) = self.channels.get(channel) {
            for mut subscriber in subscribers {

                println!("Sending message to channel: {}", channel);

                let _ = subscriber.write_all(message.as_bytes());
                subscriber.flush().unwrap();
            }
        }
    }
}
