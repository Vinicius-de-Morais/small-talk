use core::fmt;
use serde::{Deserialize, Serialize};

use crate::protocol::StatusType;

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub command: CommandTypes,
    pub input: String,
}

#[derive(Serialize, Deserialize)]
pub enum CommandTypes {
    Register,
    Login,
    Join,
    Whisper,
    Exit,
}

impl CommandTypes {
    pub fn as_str(&self) -> &str {
        match self {
            CommandTypes::Register => "/register",
            CommandTypes::Login => "/login",
            CommandTypes::Join => "/join",
            CommandTypes::Whisper => "/whisper",
            CommandTypes::Exit => "/exit",
        }
    }
}