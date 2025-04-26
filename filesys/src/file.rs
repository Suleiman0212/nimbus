use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use crate::error::FsError;

pub fn save_file_from_data(path: PathBuf, data: Vec<u8>) -> Result<(), FsError> {
    let mut file = create_file(path)?;
    file.write_all(&data)?;
    Ok(())
}

pub fn load_file_data(path: PathBuf) -> Result<Vec<u8>, FsError> {
    if !is_file_exitsts(path.clone()) {
        return Err(FsError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File {:#?} not found", path),
        )));
    }

    let mut file = load_file(path)?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn delete_file(path: PathBuf) -> Result<(), FsError> {
    if !is_file_exitsts(path.clone()) {
        return Err(FsError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File {:#?} not found", path),
        )));
    }

    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn get_file_size(path: PathBuf) -> Result<u64, FsError> {
    Ok(fs::metadata(path)?.len())
}

pub fn list_of_files(path: PathBuf) -> Result<Vec<String>, FsError> {
    let mut files = Vec::new();

    let entries = fs::read_dir(path)?;
    for entry in entries {
        let file_name = entry?.file_name();
        let file_name = file_name.to_string_lossy();
        files.push(file_name.to_string());
    }
    Ok(files)
}

fn create_file(path: PathBuf) -> Result<File, FsError> {
    Ok(File::create(path)?)
}

fn load_file(path: PathBuf) -> Result<File, FsError> {
    Ok(File::open(path)?)
}

fn is_file_exitsts(path: PathBuf) -> bool {
    path.exists()
}
