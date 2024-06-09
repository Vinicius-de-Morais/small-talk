use core::fmt;
use serde::{Deserialize, Serialize};

use crate::protocol::StatusType;

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub request_id: u64,
    pub status: StatusType,
    pub success: bool,
    pub user: i32,
    pub user_name: String,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ \"request_id\": {:?}, \"status\": \"{}\", \"success\": {}, \"user\": {}, \"user_name\": \"{}\" }}",
            self.request_id, self.status, self.success, self.user, self.user_name
        )
    }
}