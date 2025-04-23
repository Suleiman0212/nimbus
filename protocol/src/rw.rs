use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{error::MessageError, message::Message};

fn read_stream(stream: &mut TcpStream) -> Result<Vec<u8>, MessageError> {
    let mut buf = [0; 1024];
    let mut data = Vec::new();

    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            break;
        }
        data.extend_from_slice(&buf[..bytes_read]);

        if data.ends_with(b"END") {
            data.truncate(data.len() - 3);
            break;
        }
    }

    if data.len() == 0 {
        return Err(MessageError::NoDataRecieved);
    }

    Ok(data)
}

fn write_stream(stream: &mut TcpStream, mut data: Vec<u8>) -> Result<(), MessageError> {
    data.extend_from_slice(b"END");
    stream.write_all(&data)?;
    stream.flush()?;
    Ok(())
}

pub fn get_message(stream: &mut TcpStream) -> Result<Message, MessageError> {
    let data = read_stream(stream)?;
    Ok(Message::from_bytes(data)?)
}

pub fn send_message(stream: &mut TcpStream, message: Message) -> Result<(), MessageError> {
    let data = message.to_bytes();
    write_stream(stream, data?)
}
