use std::{net::TcpListener, thread};

use config::server::Config;
use server::{error::ServerError, processor::handle_connection};

fn main() -> Result<(), ServerError> {
    tracing_subscriber::fmt::init();

    let config = Config::get()?;
    let Config {
        bind_addres,
        root_dir,
        auth_required,
        users,
    } = config;

    let listener = TcpListener::bind(&bind_addres)?;
    tracing::info!("Server started listening on: {}", bind_addres);

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                let _ = thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(err) => tracing::error!("Connection error: {}", err),
        }
    }
    Ok(())
}
