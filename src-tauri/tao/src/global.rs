use crate::tauri_constants::{
    APP, HOTKEYS, HOTKEY_MANAGER, HOTKEY_RUNNING, HOTKEY_STOP_TX, MAIN_WINDOW, WINDOW_STOP_TX,
};
use common::types::{enums::WebWindow, hotkey::SafeHotKeyManager, types::Key};
use sea_orm::Iden;
use std::{collections::HashMap, sync::MutexGuard};
use tauri::{AppHandle, Manager, WebviewWindow};
use tokio::sync::oneshot;

pub fn get_main_window() -> MutexGuard<'static, WebviewWindow> {
    MAIN_WINDOW
        .get()
        .expect("Failed to get MAIN_WINDOW")
        .lock()
        .expect("Failed to lock MAIN_WINDOW")
}

pub fn get_hotkey_manager() -> MutexGuard<'static, SafeHotKeyManager> {
    HOTKEY_MANAGER
        .get()
        .expect("Failed to get HOTKEY_MANAGER")
        .lock()
        .expect("Failed to lock HOTKEY_MANAGER")
}

pub fn get_hotkey_store() -> MutexGuard<'static, HashMap<u32, Key>> {
    HOTKEYS
        .get()
        .expect("Failed to get HOTKEYS")
        .lock()
        .expect("Failed to lock HOTKEYS")
}

pub fn get_window_stop_tx() -> MutexGuard<'static, Option<oneshot::Sender<()>>> {
    WINDOW_STOP_TX
        .get()
        .expect("Failed to get WINDOW_STOP_TX")
        .lock()
        .expect("Failed to lock WINDOW_STOP_TX")
}

pub fn get_hotkey_stop_tx() -> MutexGuard<'static, Option<oneshot::Sender<()>>> {
    HOTKEY_STOP_TX
        .get()
        .expect("Failed to get HOTKEY_STOP_TX")
        .lock()
        .expect("Failed to lock HOTKEY_STOP_TX")
}

pub fn get_hotkey_running() -> MutexGuard<'static, bool> {
    HOTKEY_RUNNING
        .get()
        .expect("Failed to get HOTKEY_RUNNING")
        .lock()
        .expect("Failed to lock HOTKEY_RUNNING")
}

pub fn get_app() -> &'static AppHandle {
    APP.get().expect("Failed to get APP")
}

pub fn get_app_window(window: WebWindow) -> WebviewWindow {
    get_app()
        .get_webview_window(window.to_string().as_str())
        .expect("Failed to get webview window")
}
