use std::fmt;
use std::hash::{Hash, Hasher};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread::{self, ThreadId};
use serde::{Deserialize, Serialize};

use crate::models::User;
use crate::dto::header::Header;
use crate::dto::command::Command;


pub struct Protocol {
    status: StatusType,
}

impl Protocol {

    // pub fn new(){
    //     let status = status_type;
    //     Protocol { status }
    // }

    pub fn check_response(self, response: Response) {
        assert!(response.header.success != true);
    }

    pub fn send(addr: &str, user: User, payload: json::JsonValue) -> io::Result<()> {
        // Build the request
        let request = Protocol::build(user, payload);

        // Establish TCP connection to the server
        let mut stream = TcpStream::connect(addr)?;

        // Send the request to the server
        stream.write_all(&request)?;

        Ok(())
    }

    fn build(user: User, payload: json::JsonValue) -> Vec<u8> {
        let status_type = StatusType::Okay;

        let thread_id = thread::current().id();

        // lidar com o ID da thead ja que ela esta atualmente com bug na conversão para u64
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        thread_id.hash(&mut hasher);

        let header = Header {
            user: user.id,
            user_name: user.nickname,
            status: status_type,
            request_id: hasher.finish(),
            success: true,
        };
        let req_type = RequestType::Receive;

        let header_send = HeaderSend {
            req_type,
            header,
            payload,
        };

        let request = format!("{}\r\n", header_send);
        request.into_bytes()
    }

    pub fn handle_request(buffer_string: &[u8]) -> HeaderSend{
        assert!(!buffer_string.starts_with(b"SEND"));
        
        let (mut header_json, mut command_json) = Protocol::read_buffer(buffer_string);
        let res_payload = command_json.handle_command(&mut header_json);

        let header_send = HeaderSend {
            req_type: RequestType::Receive,
            header: header_json,
            payload: res_payload,
        };

        header_send
    }
    
    pub fn read_buffer(buffer_string: &[u8]) -> (Header, Command){
        // separar as partes da resposta
        let vec_req: Vec<&[u8]> = buffer_string
        .split(|&b| b == b'\r' || b == b'\n')
        .filter(|slice| !slice.is_empty())
        .collect();

        let mut header_slice = String::from_utf8_lossy(vec_req[1]).into_owned();
        let payload_slice = String::from_utf8_lossy(vec_req[2]).into_owned().replace("Payload:", "");

        // Ainda não decidi o que vou fazer com isso
        let mut header_json: Header = serde_json::from_str(&header_slice).expect("Failed to parse header from JSON");
        let mut command_json: Command = serde_json::from_str(&payload_slice).expect("Failed to parse header from JSON");
        
        (header_json, command_json)
    }
}

pub struct HeaderSend {
    req_type: RequestType,
    header: Header,
    payload: json::JsonValue,
}

pub enum RequestType {
    Receive, // get
    Send,    // post
}

impl fmt::Display for RequestType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestType::Receive => write!(f, "RECEIVE"),
            RequestType::Send => write!(f, "SEND"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum StatusType {
    Okay,
    Error,
    MissingArg,
}

impl fmt::Display for StatusType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatusType::Okay => write!(f, "Okay"),
            StatusType::Error => write!(f, "Error"),
            StatusType::MissingArg => write!(f, "MissingArg"),
        }
    }
}

impl fmt::Display for HeaderSend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\r\n{}\r\nPayload: {}",
            self.req_type, self.header, self.payload
        )
    }
}

pub struct Request {
    header: Header,
    content: String,
}

pub struct Response {
    header: Header,
    content: String,
}
