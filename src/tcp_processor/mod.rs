use core::f32;
use std::{
    error::Error,
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream},
};

use dtp::{Content, ContentType, Message, SubTitile, Title};
use rw::{send_message, wait_ok};

pub fn connect_to_server(server_ip: String) -> Result<TcpStream, Box<dyn Error>> {
    let addr = server_ip.parse::<SocketAddr>()?;
    Ok(TcpStream::connect(addr)?)
}

// Get request like a "Download"
// Steps:
// 1. Sending file name
// 2. Getting file size
// 3. Waiting for user decision
// 4. If alls good, sending OK
// 5. Collecting file binary data
// and writing it to a file
pub fn get_request(stream: &mut TcpStream, file_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let name_content = Content::Text(file_name.to_string());
    let message: Message = Message::new(
        Title::GetRequest,
        SubTitile::Ok,
        ContentType::FileName,
        vec![name_content],
    );

    rw::send_message(stream, message)?;

    let file_size_msg = rw::get_message(stream)?;
    let file_size = unbox_message(file_size_msg, Title::GetRequest, ContentType::FileSize)?;
    let file_size = match file_size[0] {
        Content::Number(n) => n,
        _ => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "InvalidType: file content incorrect type (isnt Binary).",
            )))
        }
    };

    let fs_mb: f32 = file_size as f32 / 1024.0 / 1024.0;

    println!(
        "\nFile \"{file_name}\"\nWill be downloaded: {fs_mb}mb\n\n:::Want to download? [Y/n] "
    );
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    match input {
        "" | "y" | "Y" => (),
        "n" | "N" => {
            rw::send_err(stream, Title::GetRequest, "Aborted by user.")?;
            std::process::exit(0);
        }
        _ => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Aborted, input Y or N!",
            )));
        }
    }

    rw::send_ok(stream, Title::GetRequest)?;

    let file_message = rw::get_message(stream)?;
    let file_data =
        match unbox_message(file_message, Title::GetRequest, ContentType::FileData)?[0].clone() {
            Content::Binary(b) => b,
            _ => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    "InvalidType: file content incorrect type (isnt Binary).",
                )))
            }
        };

    Ok(file_data)
}

// Send request like a "Upload"
// Steps:
// 1. Sending request with file name
// 2. Waiting for OK
// 3. Sending file binary data
// 4. Waiting for OK
pub fn send_request(
    stream: &mut TcpStream,
    file_name: &str,
    file_data: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    let name_content = Content::Text(file_name.to_string());
    let message: Message = Message::new(
        Title::SendRequest,
        SubTitile::Ok,
        ContentType::FileName,
        vec![name_content],
    );

    rw::send_message(stream, message)?;

    rw::wait_ok(stream, Title::SendRequest)?;

    // TODO: Move file read functionality to main.rs

    let file_data = Content::Binary(file_data);

    let file_message: Message = Message::new(
        Title::SendRequest,
        SubTitile::Ok,
        ContentType::FileData,
        vec![file_data],
    );

    send_message(stream, file_message)?;

    wait_ok(stream, Title::SendRequest)?;
    println!("File uploaded!");

    Ok(())
}

pub fn file_list_get_request(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let message: Message = Message::new(
        Title::FileListRequest,
        SubTitile::Ok,
        ContentType::NoContent,
        vec![],
    );
    rw::send_message(stream, message)?;

    let response = rw::get_message(stream)?;
    let response =
        match unbox_message(response, Title::FileListRequest, ContentType::FileData)?[0].clone() {
            Content::Binary(b) => b,
            _ => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    "InvalidType: file content incorrect type (isnt Binary).",
                )))
            }
        };

    Ok(String::from_utf8_lossy(&response).to_string())
}

// Unboxing message like a gift
// It needed to catch any errors
// Related with incorrect types
fn unbox_message(
    message: Message,
    ok_title: Title,
    ok_content_type: ContentType,
) -> Result<Vec<Content>, Box<dyn Error>> {
    if message.title != ok_title {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "IncorrectMessage: title content incorrect type.",
        )));
    }

    match message.sub_title {
        SubTitile::Ok => (),
        _ => {
            let e = match message.content_array[0].clone() {
                Content::Text(t) => t,
                _ => "<Cant read error message, incorrect type.>".to_string(),
            };
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("ErrorMessage: {e}"),
            )));
        }
    }

    if message.content_type != ok_content_type {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "IncorrectMessage: content type is incorrect.",
        )));
    }

    Ok(message.content_array)
}
