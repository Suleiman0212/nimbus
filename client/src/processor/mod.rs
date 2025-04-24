use std::net::TcpStream;

use crate::{ClientHandle, error::ClientError};
use config::client::User;
use protocol::{message::Message, rw};

pub fn connect_to_server(server_addres: String) -> Result<TcpStream, ClientError> {
    Ok(TcpStream::connect(server_addres)?)
}

pub fn process_login(stream: &mut TcpStream, User { login, password }: User) -> ClientHandle {
    let message = Message::LoginRequest { login, password };
    rw::send_message(stream, message)?;
    let answer = rw::get_message(stream)?;
    return match answer {
        Message::LoginAnswer { login_successfully } => {
            if login_successfully {
                Ok(())
            } else {
                Err(ClientError::IncorrectUser)
            }
        }
        _ => Err(ClientError::IncorrectMessage),
    };
}

pub fn process_session(stream: &mut TcpStream) -> ClientHandle {
    let message = Message::LoginSessionRequest;
    rw::send_message(stream, message)?;
    let answer = rw::get_message(stream)?;
    return match answer {
        Message::LoginSessionAnswer { session_exists } => {
            if session_exists {
                Ok(())
            } else {
                Err(ClientError::IncorrectUser)
            }
        }
        _ => Err(ClientError::IncorrectMessage),
    };
}
