use std::net::TcpStream;

use crate::{CONFIG, HandleResult, add_session, session_exists};
use protocol::{message::Message, rw};

pub fn handle_connection(mut stream: TcpStream) -> HandleResult {
    let message = rw::get_message(&mut stream);
    match message {
        Ok(msg) => handle_message(stream, msg)?,
        Err(err) => {
            tracing::error!("{}", err);
        }
    }
    Ok(())
}

fn handle_message(stream: TcpStream, message: Message) -> HandleResult {
    match message {
        Message::LoginRequest { login, password } => handle_login_request(stream, login, password)?,
        Message::LoginSessionRequest => handle_login_session_request(stream)?,
        _ => tracing::warn!("Unhandled message: {:#?}", message),
    }
    Ok(())
}

fn handle_login_request(mut stream: TcpStream, login: String, password: String) -> HandleResult {
    let login_successfully = CONFIG
        .users
        .iter()
        .any(|u| (u.login == login) && (u.password == password));
    let message = Message::LoginAnswer { login_successfully };

    rw::send_message(&mut stream, message)?;
    if login_successfully {
        add_session(stream.peer_addr()?);
        tracing::info!("User {} login successfully", login);
    } else {
        tracing::info!("User {} unsuccessful login", login);
    }
    Ok(())
}

fn handle_login_session_request(mut stream: TcpStream) -> HandleResult {
    let session_exists = session_exists(stream.peer_addr()?);
    let message = Message::LoginSessionAnswer { session_exists };
    rw::send_message(&mut stream, message)?;
    Ok(())
}
