use small_talk::models::*;

pub struct Protocol{
    status: status_type,
}

pub impl Protocol{
    pub fn new(){
        let status = status_type;

        Protocol{status}
    }

    pub fn check_response(self, response: Response){
        assert!(response.header)
    }
}

pub enum status_type {
    ok(String),
    error(String),
    missing_arg(String)
}

pub struct Request{
    header: Header,
    content: String,
}

pub struct Response {
    header: Header,
    content: String,
}

pub struct Header{
    request_id: ThreadId,
    status: status_type,
    success: bool,
    user: User::id,
}


