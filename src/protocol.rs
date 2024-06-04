use std::fmt;
use std::io::{self, Write};
use std::net::TcpStream;
use std::thread::{self, ThreadId};

use json::{self, object};

use crate::models::User;

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
        let header = Header {
            user: user.id,
            status: status_type,
            request_id: thread::current().id(),
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

pub struct Header {
    request_id: ThreadId,
    status: StatusType,
    success: bool,
    user: i32,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {:?}, Status: {}, Success: {}, User ID: {}",
            self.request_id, self.status, self.success, self.user
        )
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
