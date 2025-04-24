use std::{net::TcpListener, thread};

use server::CONFIG;
use server::{error::ServerError, processor::handle_connection};

fn main() -> Result<(), ServerError> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind(&CONFIG.bind_addres)?;
    tracing::info!("Server started listening on: {}", &CONFIG.bind_addres);

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                let _ = thread::spawn(move || {
                    match handle_connection(stream) {
                        Ok(_) => tracing::debug!("Connection handled succesfully"),
                        Err(err) => tracing::error!("{}", err),
                    };
                });
            }
            Err(err) => tracing::error!("Connection error: {}", err),
        }
    }
    Ok(())
}
