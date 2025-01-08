use config::Config;
use fs::{self};
use std::{
    env,
    error::Error,
    io::{self, Read, Write},
    path::PathBuf,
};

mod config;
mod tcp_processor;

// Collecting any args from env
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Check config file for existance
    // If it isnt, create it
    let conf: Config = match fs::is_file_exist(&config::get_config_path()?.join("config.toml")) {
        Ok(_) => config::get_config()?,
        Err(e) => {
            eprintln!("{e}");
            config::crete_conf()?;
            println!(
                "Config created in {}",
                &config::get_config_path()?.to_str().unwrap()
            );
            println!("Check and edit config data_path.");
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
            let file_name = get_param(&args, 3, false)?;
            let path_to = get_param(&args, 4, true)?;
            let path = PathBuf::new().join(path_to).join(file_name);
            download_file(path, server_ip)?;
        }
        "--upload" | "-u" => {
            let file_name = get_param(&args, 3, false)?;
            let path_from = get_param(&args, 4, true)?;
            let path = PathBuf::new().join(path_from).join(file_name);

            upload_file(path, server_ip)?;
        }
        "--file-list" | "-fl" => {
            get_file_list(server_ip)?;
        }
        "--remove" | "-rm" => {
            let file_name = get_param(&args, 3, false)?;
            let path_to = get_param(&args, 4, true)?;
            let path = PathBuf::new().join(path_to).join(file_name);

            remove_file(path, server_ip)?;
        }
        _ => exit_with("Unknown option."),
    }

    Ok(())
}

// Get patams from args if it needed
// And catching any errors
fn get_param(args: &Vec<String>, param_num: usize, optional: bool) -> Result<&str, Box<dyn Error>> {
    if args.len() > param_num - 1 {
        Ok(&args[param_num - 1])
    } else {
        if optional {
            return Ok("");
        }

        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "You need to input param!",
        )));
    }
}

// Connecting to server and call get_request
fn download_file(file_path: PathBuf, server_ip: String) -> Result<(), Box<dyn Error>> {
    let absolute_path = resolve_path(file_path)?;

    let mut stream = tcp_processor::connect_to_server(server_ip)?;

    let file_name = absolute_path.file_name().unwrap().to_str().unwrap();
    let file_data = tcp_processor::get_request(&mut stream, &file_name)?;
    let mut file = fs::create_file(&absolute_path)?;
    file.write_all(&file_data)?;

    println!("File downloaded!");

    Ok(())
}

// Connecting to server and call send_request
fn upload_file(file_path: PathBuf, server_ip: String) -> Result<(), Box<dyn Error>> {
    let absolute_path = resolve_path(file_path)?;
    fs::is_file_exist(&absolute_path)?;

    let file_name = absolute_path.file_name().unwrap().to_str().unwrap();
    let mut stream = tcp_processor::connect_to_server(server_ip)?;
    let mut buf: Vec<u8> = vec![];
    let mut file = fs::load_file(&absolute_path)?;
    file.read_to_end(&mut buf)?;
    tcp_processor::send_request(&mut stream, &file_name, buf)?;

    println!("File uploaded!");

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

fn remove_file(file_path: PathBuf, server_ip: String) -> Result<(), Box<dyn Error>> {
    let absolute_path = resolve_path(file_path)?;

    let file_name = absolute_path.file_name().unwrap().to_str().unwrap();
    let mut stream = tcp_processor::connect_to_server(server_ip)?;
    tcp_processor::remove_request(&mut stream, file_name)?;
    println!("Path {}", absolute_path.to_str().unwrap());
    println!("Removed!");
    Ok(())
}

// Converts input_path to to path_buf
// and if it isnt absolute, make it is
fn resolve_path(input_path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    if input_path == PathBuf::from("") {
        return Ok(env::current_dir()?);
    }

    if input_path.is_absolute() {
        Ok(input_path)
    } else {
        let current_dir = env::current_dir()?;
        Ok(current_dir.join(input_path).to_path_buf())
    }
}

// Info with shows when programm
// Opened with --help or -h
fn help_info() {
    println!(
        "
    Example: nimbus <option> <param> <param>
<options>:
    --help      |  -h   to show this.
    --download  |  -d   to download file from server.
    --upload    |  -u   to upload file on server.
    --file-list |  -fl  to get file list from server.
    --remove    |  -rm  to remove file/dir from server.
        "
    )
}

// Emergency exit shortcut
// Better than panic! cause it have
// Smaller output and exit code
fn exit_with(err: &str) {
    eprint!("{err}");
    std::process::exit(1);
}
