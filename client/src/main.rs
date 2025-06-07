use client::{CONFIG, error::ClientError, processor};
use std::env;

fn main() -> Result<(), ClientError> {
    let args: Vec<String> = env::args().collect();
    execute_commands(args)?;
    Ok(())
}

fn execute_commands(args: Vec<String>) -> Result<(), ClientError> {
    let server_address = CONFIG.server_addres.clone();

    match args.get(1).map(String::as_str) {
        Some("--login") => login(server_address)?,
        Some("--session") => session(server_address)?,
        Some("--meta") => {
            let file_path = args.get(2).ok_or(ClientError::InvalidArguments)?;
            meta(server_address, file_path.clone())?
        }
        Some("--download") => {
            let file_path = args.get(2).ok_or(ClientError::InvalidArguments)?;
            download(server_address, file_path.clone())?
        }
        _ => {
            println!("Unknown command.");
        }
    }
    Ok(())
}

fn login(server_address: String) -> Result<(), ClientError> {
    let mut stream = processor::connect_to_server(server_address)?;
    processor::process_login(&mut stream, CONFIG.user.clone())?;
    println!("Login successfully!");
    Ok(())
}

fn session(server_address: String) -> Result<(), ClientError> {
    let mut stream = processor::connect_to_server(server_address)?;
    processor::process_session(&mut stream)?;
    println!("Session exists!");
    Ok(())
}

fn meta(server_address: String, file_path: String) -> Result<(), ClientError> {
    let mut stream = processor::connect_to_server(server_address)?;

    let mut file_size = 0;
    processor::process_login(&mut stream, CONFIG.user.clone())?;
    processor::process_session(&mut stream)?;
    processor::process_meta_download(&mut stream, file_path.clone(), &mut file_size)?;
    println!("File size: {} KB", file_size / 1024);
    Ok(())
}

fn download(server_address: String, file_path: String) -> Result<(), ClientError> {
    let mut stream = processor::connect_to_server(server_address)?;

    processor::process_login(&mut stream, CONFIG.user.clone())?;
    processor::process_session(&mut stream)?;

    let mut file_size = 0;
    processor::process_meta_download(&mut stream, file_path.clone(), &mut file_size)?;
    println!("File size: {} KB", file_size / 1024);

    await_agree("Download file?");

    let mut file_data = Vec::new();
    processor::process_download(&mut stream, file_path, &mut file_data)?;
    filesys::file::save_file_from_data("/home/".into(), file_data)?;
    println!("File downloaded!");
    Ok(())
}

fn await_agree(message: &str) {
    println!("{} [Y/n]:", message);
    let mut agree = String::new();
    std::io::stdin().read_line(&mut agree).unwrap();
    let agree = agree.trim(); // Убрать \n

    if agree == "Y" || agree == "y" || agree.is_empty() {
        return;
    } else {
        std::process::exit(0);
    }
}
