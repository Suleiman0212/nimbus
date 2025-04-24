use std::{env, net::TcpStream};

use client::{CONFIG, ClientHandle, error::ClientError, processor};

fn main() -> ClientHandle {
    let args: Vec<String> = env::args().collect();
    execute_commands(args)?;

    Ok(())
}

fn execute_commands(args: Vec<String>) -> ClientHandle {
    let server_addres = CONFIG.server_addres.clone();
    let stream = processor::connect_to_server(server_addres)?;

    match args[1].as_ref() {
        "--login" => login(stream)?,
        "--session" => session(stream)?,
        _ => (),
    }
    Ok(())
}

fn login(mut stream: TcpStream) -> Result<(), ClientError> {
    let user = CONFIG.user.clone();
    processor::process_login(&mut stream, user)?;
    println!("login succesfully!");
    Ok(())
}

fn session(mut stream: TcpStream) -> ClientHandle {
    processor::process_session(&mut stream)?;
    println!("session exists!");
    Ok(())
}
