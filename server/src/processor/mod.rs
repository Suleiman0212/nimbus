use std::{net::TcpStream, path::PathBuf};

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
        Message::FileMetaRequest { file_path } => handle_file_meta_request(stream, file_path)?,
        Message::FileDownloadRequest { file_path } => {
            handle_file_download_request(stream, file_path)?
        }
        Message::FileUploadRequest {
            file_path,
            file_data,
        } => handle_file_upload_request(stream, file_path, file_data)?,
        Message::FileDeleteRequest { file_path } => handle_file_delete_request(stream, file_path)?,
        Message::FileListRequest { path } => handle_file_list_request(stream, path)?,
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
        tracing::debug!("User {} login successfully", login);
    } else {
        tracing::debug!("User {} unsuccessful login", login);
    }
    Ok(())
}

fn handle_login_session_request(mut stream: TcpStream) -> HandleResult {
    let session_exists = session_exists(stream.peer_addr()?);
    let message = Message::LoginSessionAnswer { session_exists };
    rw::send_message(&mut stream, message)?;
    Ok(())
}

fn handle_file_meta_request(mut stream: TcpStream, file_path: String) -> HandleResult {
    let file_size = Ok(filesys::file::get_file_size(file_path.into())?);
    let message = Message::FileMetaAnswer { file_size };
    rw::send_message(&mut stream, message)?;
    Ok(())
}

fn handle_file_download_request(mut stream: TcpStream, file_path: String) -> HandleResult {
    let path = join_path(file_path);

    let file_data = Ok(filesys::file::load_file_data(path)?);
    let message = Message::FileDownloadAnswer { file_data };
    rw::send_message(&mut stream, message)?;
    Ok(())
}

fn handle_file_upload_request(
    mut stream: TcpStream,
    file_path: String,
    file_data: Vec<u8>,
) -> HandleResult {
    let path = join_path(file_path);

    let uploaded = match filesys::file::save_file_from_data(path, file_data) {
        Ok(_) => true,
        Err(err) => {
            tracing::error!("{}", err);
            false
        }
    };
    let message = Message::FileUploadAnswer { uploaded };
    rw::send_message(&mut stream, message)?;
    Ok(())
}

fn handle_file_delete_request(mut stream: TcpStream, file_path: String) -> HandleResult {
    let path = join_path(file_path);

    let deleted = Ok(match filesys::file::delete_file(path) {
        Ok(_) => true,
        Err(err) => {
            tracing::error!("{}", err);
            false
        }
    });
    let message = Message::FileDeleteAnswer { deleted };
    rw::send_message(&mut stream, message)?;
    Ok(())
}

fn handle_file_list_request(mut stream: TcpStream, file_path: String) -> HandleResult {
    let path = join_path(file_path);
    let files = filesys::file::list_of_files(path)?;
    let message = Message::FileListAnswer { files };
    rw::send_message(&mut stream, message)?;
    Ok(())
}

fn join_path(file_path: String) -> PathBuf {
    let path: PathBuf = CONFIG.root_dir.clone().into();
    let path = path.join(file_path);
    path
}
