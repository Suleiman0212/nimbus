use std::net::TcpStream;

use crate::error::ClientError;
use config::client::User;
use protocol::{message::Message, rw};

pub fn connect_to_server(server_addres: String) -> Result<TcpStream, ClientError> {
    Ok(TcpStream::connect(server_addres)?)
}

pub fn process_login(
    stream: &mut TcpStream,
    User { login, password }: User,
) -> Result<(), ClientError> {
    let message = Message::LoginRequest { login, password };
    rw::send_message(stream, message)?;
    let recieved = rw::get_message(stream)?;
    return match recieved {
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
