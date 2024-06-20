use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::{conn::establish_connection, models::User, protocol::StatusType};

use super::header::Header;

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub command: CommandTypes,
    pub input: String,
}

impl Command {
    pub fn handle_command(self, header: &mut Header) -> json::JsonValue {
        match self.command {
            CommandTypes::Register => Self::register(self, header),
            CommandTypes::Login => Self::login(self, header),
            CommandTypes::Join => Self::join(),
            CommandTypes::Message => Self::message(self, header),
            CommandTypes::Whisper => Self::whisper(),
            CommandTypes::Exit => Self::exit(),
        }
    }

    // alteramos o user q foi passado para o registrado
    fn register(self, header: &mut Header) -> json::JsonValue {
        // por enquanto vou só receber o nick no input

        let mut connection = establish_connection();

        //create user
        let user: User =
            User::register(&mut connection, self.input.as_str()).expect("Fail to create user");

        header.user = user.id;
        header.user_name = user.nickname;

        json::object! {
            "status" => "success",
        }
    }

    fn login(self, header: &mut Header) -> json::JsonValue {
        let mut connection = establish_connection();

        let res_obj: json::JsonValue;
        //create user
        match User::find_user(&mut connection, self.input.to_owned()) {
            Ok(user) => {
                let user_name: String = user.nickname.clone();

                header.user = user.id;
                header.user_name = user.nickname;

                res_obj = json::object! {
                    "status" => "success",
                    "user_name" => user_name,
                };
            }
            Err(err) => {
                header.user = 0;
                header.user_name = "".to_owned();
                header.success = false;
                header.status = StatusType::Error;

                res_obj = json::object! {
                    "status" => "error".to_owned(),
                    "message" => err.to_string(),
                };
            }
        }
        res_obj
    }

    fn join() -> json::JsonValue {
        
        json::object! {
            "status" => "not_implemented",
        } 
    }

    fn message(self, header: &mut Header) -> json::JsonValue {

        json::object! {
            "message" => self.input,
            "channel" => <std::string::String as Clone>::clone(&header.channel)
        } 
    }

    fn whisper() -> json::JsonValue {

        json::object! {
            "status" => "not_implemented",
        } 
    }

    fn exit() -> json::JsonValue {

        json::object! {
            "status" => "not_implemented",
        } 
    }
}

#[derive(Serialize, Debug)]
pub enum CommandTypes {
    Register,
    Login,
    Join,
    Message,
    Whisper,
    Exit,
}

impl CommandTypes {
    pub fn as_str(&self) -> &str {
        match self {
            CommandTypes::Register => "/register",
            CommandTypes::Login => "/login",
            CommandTypes::Join => "/join",
            CommandTypes::Message => "/message",
            CommandTypes::Whisper => "/whisper",
            CommandTypes::Exit => "/exit",
        }
    }
}

// IMPLEMENTAÇÃO PARA CORRIGIR O JSON
impl<'de> Deserialize<'de> for CommandTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommandVisitor;

        impl<'de> Visitor<'de> for CommandVisitor {
            type Value = CommandTypes;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid command string")
            }

            fn visit_str<E>(self, value: &str) -> Result<CommandTypes, E>
            where
                E: de::Error,
            {
                match value {
                    "/register" => Ok(CommandTypes::Register),
                    "/login" => Ok(CommandTypes::Login),
                    "/join" => Ok(CommandTypes::Join),
                    "/message" => Ok(CommandTypes::Message),
                    "/whisper" => Ok(CommandTypes::Whisper),
                    "/exit" => Ok(CommandTypes::Exit),
                    _ => Err(de::Error::unknown_variant(
                        value,
                        &["/register", "/login", "/join", "/whisper", "/exit"],
                    )),
                }
            }
        }

        deserializer.deserialize_str(CommandVisitor)
    }
}
