use crate::service::{
    clipboard::count_clipboards_db, settings::get_data_path, window::open_window,
};
use common::types::{
    enums::WebWindow,
    types::{CommandError, Config, DatabaseInfo},
};
use std::fs::{self, read_to_string};
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub async fn open_new_window(window_name: WebWindow) {
    open_window(window_name).await;
}

#[tauri::command]
pub fn exit_app() {
    std::process::exit(0);
}

#[tauri::command]
pub fn get_app_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}

#[tauri::command]
pub fn open_browser_url(url: String, app: AppHandle) -> Result<(), CommandError> {
    Ok(app.opener().open_url(url, None::<String>)?)
}

#[tauri::command]
pub async fn get_db_info() -> Result<DatabaseInfo, CommandError> {
    let data_path = get_data_path();

    let config: Config = serde_json::from_str(&read_to_string(&data_path.config_file_path)?)?;
    let size = fs::metadata(config.db)?.len();

    let records = count_clipboards_db().await?;

    Ok(DatabaseInfo { records, size })
}

#[tauri::command]
pub async fn get_db_path() -> Result<String, CommandError> {
    let data_path = get_data_path();
    let config: Config = serde_json::from_str(&read_to_string(&data_path.config_file_path)?)?;
    Ok(config.db)
}
