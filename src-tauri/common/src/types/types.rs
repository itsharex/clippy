use global_hotkey::hotkey::HotKey;
use google_drive3::yup_oauth2;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub db: String,
    #[serde(default)]
    pub drive_token: Option<yup_oauth2::AccessToken>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPath {
    pub config_path: String,
    pub db_file_path: String,
    pub config_file_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseInfo {
    pub records: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Key {
    pub id: u32,
    pub state: bool,
    pub is_global: bool,
    pub key_str: String,
    pub event: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub key: String,
    pub hotkey: HotKey,
}

#[derive(Debug)]
pub enum KeyboardLayout {
    Qwerty,
    Qwertz,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandError {
    Error(String),
}

// Simplified error handling using a single variant
impl<E: std::error::Error> From<E> for CommandError {
    fn from(err: E) -> Self {
        CommandError::Error(err.to_string())
    }
}
