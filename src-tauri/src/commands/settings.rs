use entity::settings::Model;

use crate::{service::settings::{get_settings_db, update_settings_db}, utils::hotkey::hotkey_listener::init_hotkey_listener};

#[tauri::command]
pub async fn get_settings() -> Result<Model, String> {
    let res = get_settings_db().await;

    Ok(res.unwrap())
}

#[tauri::command]
pub async fn update_settings(settings: Model) -> Result<Model, String> {
    let res = update_settings_db(settings).await;

    init_hotkey_listener(false);

    Ok(res.unwrap())
}
