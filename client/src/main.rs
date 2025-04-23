use std::env;

use client::{error::ClientError, processor};
use config::client::Config;

fn main() -> Result<(), ClientError> {
    let config = Config::get()?;

    let args: Vec<String> = env::args().collect();
    execute_commands(args, config)?;

    Ok(())
}

fn execute_commands(args: Vec<String>, config: Config) -> Result<(), ClientError> {
    match args[1].as_ref() {
        "--login" => login(config)?,
        _ => (),
    }
    Ok(())
}

fn login(
    Config {
        server_addres,
        user,
    }: Config,
) -> Result<(), ClientError> {
    let mut stream = processor::connect_to_server(server_addres)?;
    processor::process_login(&mut stream, user)?;
    println!("login succesfully!");
    Ok(())
}
