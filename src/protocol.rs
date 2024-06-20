use std::fmt;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{self};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use crate::models::User;
use crate::dto::header::Header;
use crate::dto::command::Command;


pub struct Protocol {
    //status: StatusType,
}

impl Protocol {

    // METODOS SENT E BUIL UTILIZADOS MAJORITARIAMENTE PARA TESTE

    pub fn send(user: User, payload: json::JsonValue) -> io::Result<String> {
        // Build the request
        let request = Protocol::build(user, payload);

        Ok(request.to_string())
    }

    fn build(user: User, payload: json::JsonValue) -> HeaderSend {
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
            channel: '/'.to_string()
        };

        let req_type = RequestType::Send;

        let header_send = HeaderSend {
            req_type,
            header,
            payload,
        };

        header_send
    }

    // lidar coom o request lendo os dados vindos do buffer para trasnformar num header utilizavel
    pub fn handle_request(buffer_string: &[u8]) -> HeaderSend{
        //assert!(!buffer_string.starts_with(b"SEND"));
        
        let (mut header_json, command_json) = Protocol::get_header_and_command(buffer_string);
        let res_payload = command_json.handle_command(&mut header_json);

        let mut req_type = RequestType::Receive;
        if !res_payload["channel"].is_null() {
            req_type = RequestType::Send;
        }

        let header_send = HeaderSend {
            req_type: req_type,
            header: header_json,
            payload: res_payload,
        };

        header_send
    }

    // Transformar o buffer em estruturas do rust, para ler de maneira correta os headers e requests
    pub fn get_header_and_command(buffer_string: &[u8]) -> (Header, Command){

        let (header, command) = Protocol::read_buffer(buffer_string);

        println!("Header: {}", header);
        println!("Command: {}", command);

        let header_json = Protocol::header_to_object(&header);
        let command_json = Protocol::command_to_object(&command);

        (header_json, command_json)
    }


    pub fn read_buffer(buffer_string: &[u8]) -> (String, String){
        // separar as partes da resposta
        let vec_req: Vec<&[u8]> = buffer_string
        .split(|&b| b == b'\r' || b == b'\n')
        .filter(|slice| !slice.is_empty())
        .collect();

        let header_slice = String::from_utf8_lossy(vec_req[1]).into_owned();
        let payload_slice = String::from_utf8_lossy(vec_req[2]).into_owned().replace("Payload:", "");
        
        (header_slice, payload_slice)
    }

    pub fn header_to_object(header: &str) -> Header {
        let header_json: Header = serde_json::from_str(header).unwrap();
        header_json
    }

    pub fn command_to_object(command: &str) -> Command {
        // let command_json: Command = serde_json::from_str(command).unwrap();
        let command_json: Command = Command::deserialize(&mut Deserializer::from_str(command)).unwrap();
        command_json
    }

}

pub struct HeaderSend {
    pub req_type: RequestType,
    pub header: Header,
    pub payload: json::JsonValue,
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

impl PartialEq for RequestType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RequestType::Receive, RequestType::Receive) => true,
            (RequestType::Send, RequestType::Send) => true,
            _ => false,
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

// pub struct Request {
//     header: Header,
//     content: String,
// }

// pub struct Response {
//     header: Header,
//     content: String,
// }
