use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Mutex,
    time::{Duration, SystemTime},
};

use config::server::Config;
use error::ServerError;
use once_cell::sync::Lazy;

pub mod error;
pub mod processor;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::get().unwrap());
pub static SESSIONS: Lazy<Mutex<HashMap<String, SystemTime>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
type HandleResult = Result<(), ServerError>;

pub fn add_session(peer_addres: SocketAddr) {
    let mut sessions = SESSIONS.lock().unwrap();
    sessions.insert(peer_addres.ip().to_string(), SystemTime::now());
}

pub fn session_exists(peer_addres: SocketAddr) -> bool {
    let sessions = SESSIONS.lock().unwrap();
    // cleanup_expired_sessions();
    if let Some(timestamp) = sessions.get(&peer_addres.ip().to_string()) {
        timestamp.elapsed().unwrap_or(Duration::from_secs(0)) < Duration::from_secs(300)
    } else {
        false
    }
}
pub fn cleanup_expired_sessions() {
    let mut sessions = SESSIONS.lock().unwrap();
    let now = SystemTime::now();
    sessions.retain(|_, time| {
        now.duration_since(*time).unwrap_or(Duration::from_secs(0)) < Duration::from_secs(300)
    });
}
