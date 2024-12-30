use config::Config;
use fs;
use std::{env, error::Error, io};

mod config;
mod tcp_processor;

// Collecting any args from env
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Check config file for existance
    // If it isnt, create it
    let conf: Config = match fs::is_file_exist(&config::get_config_path()?, "config.toml") {
        Ok(_) => config::get_config()?,
        Err(e) => {
            eprintln!("{e}");
            config::crete_conf()?;
            println!("Config created in {}", &config::get_config_path()?);
            println!("Chech and edit config data_path.");
            config::get_config()?
        }
    };

    execute_args(args, conf.server_ip)?;
    Ok(())
}

// Parse args and call functions
fn execute_args(args: Vec<String>, server_ip: String) -> Result<(), Box<dyn Error>> {
    let option: &str = &args[1];

    match option {
        "--help" | "-h" => help_info(),
        "--download" | "-d" => {
            let param = get_param(&args);
            download_file(param?, server_ip)?;
        }
        "--upload" | "-u" => {
            let param = get_param(&args);
            upload_file(param?, server_ip)?;
        }
        "--file-list" | "-fl" => {
            get_file_list(server_ip)?;
        }
        _ => exit_with("Unknown option."),
    }

    Ok(())
}

// Get patams from args if it needed
// And catching any errors
fn get_param(args: &Vec<String>) -> Result<&str, Box<dyn Error>> {
    if args.len() > 2 {
        Ok(&args[2])
    } else {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "You need to input param!",
        )));
    }
}

// Connecting to server and call get_request
fn download_file(file_name: &str, server_ip: String) -> Result<(), Box<dyn Error>> {
    let mut stream = tcp_processor::connect_to_server(server_ip)?;
    tcp_processor::get_request(&mut stream, file_name)?;
    Ok(())
}

// Connecting to server and call send_request
fn upload_file(file_name: &str, server_ip: String) -> Result<(), Box<dyn Error>> {
    fs::is_file_exist(tcp_processor::FILE_DIR, file_name)?;
    let mut stream = tcp_processor::connect_to_server(server_ip)?;
    tcp_processor::send_request(&mut stream, file_name)?;
    Ok(())
}

fn get_file_list(server_ip: String) -> Result<(), Box<dyn Error>> {
    let mut stream = tcp_processor::connect_to_server(server_ip)?;
    let info = tcp_processor::file_list_get_request(&mut stream)?;
    println!(":::Files on server:::\n");
    println!("{info}");
    println!(":::Files on server:::");
    Ok(())
}

// Info with shows when programm
// Opened with --help or -h
fn help_info() {
    println!(
        "nimbus <option> <param>\n<options>:\n --help | -h to show this.\n --download | -d to download file from server.\n --upload | -u to upload file on server."
    )
}

// Emergency exit shortcut
// Better than panic! cause it have
// Smaller output and exit code
fn exit_with(err: &str) {
    eprint!("{err}");
    std::process::exit(1);
}
