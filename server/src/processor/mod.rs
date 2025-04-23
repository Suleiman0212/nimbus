use std::net::TcpStream;

use protocol::{message::Message, rw};

pub fn handle_connection(mut stream: TcpStream) {
    let message = rw::get_message(&mut stream);
    match message {
        Ok(msg) => handle_message(stream, msg),
        Err(err) => {
            tracing::error!("{}", err);
        }
    }
}

fn handle_message(stream: TcpStream, message: Message) {
    match message {
        Message::LoginRequest { login, password } => handle_login_request(stream, login, password),
        _ => tracing::warn!("Unhandled message: {:#?}", message),
    }
}

fn handle_login_request(mut stream: TcpStream, login: String, password: String) {
    let message = Message::LoginAnswer {
        login_successfully: true,
    };
    rw::send_message(&mut stream, message).unwrap();
    tracing::info!("Login: {}", login);
    tracing::info!("password: {}", password);
}
