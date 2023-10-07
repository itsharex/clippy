use core::time::Duration;
use tauri::WindowEvent;
use tokio::sync::oneshot;

use crate::utils::{tauri::config::{MAIN_WINDOW, WINDOW_STOP_TX}, hotkey_manager::unregister_hotkeys};

pub fn window_event_listener() {
    tauri::async_runtime::spawn(async move {
        let window = MAIN_WINDOW.get().unwrap();

        window
            .lock()
            .unwrap()
            .on_window_event(move |event| match event {
                WindowEvent::Focused(true) => {
                    window
                        .lock()
                        .unwrap()
                        .emit("set_global_hotkey_event", true)
                        .unwrap();

                    let (tx, rx) = oneshot::channel();

                    tauri::async_runtime::spawn(async {
                        let result = tokio::time::timeout(Duration::from_secs(5), rx).await;
                        match result {
                            Ok(_) => return, // If we're signaled, exit early
                            Err(_) => {
                                // Acquire the lock only when you need it
                                window
                                    .lock()
                                    .unwrap()
                                    .emit("set_global_hotkey_event", false)
                                    .unwrap();
                                unregister_hotkeys(false);
                            }
                        }
                    });

                    // Store the sender in the WINDOW_STOP_TX global variable
                    *WINDOW_STOP_TX.get().unwrap().lock().unwrap() = Some(tx);
                }
                WindowEvent::Focused(false) => {
                    tauri::async_runtime::spawn(async {
                        std::thread::sleep(Duration::from_millis(200));

                        // Use the sender to signal the timer thread to exit early
                        if let Some(tx) = WINDOW_STOP_TX.get().unwrap().lock().unwrap().take() {
                            let _ = tx.send(());
                        }

                        // toggle_main_window(Some(false));
                        unregister_hotkeys(false);

                        window.lock().unwrap().hide().unwrap();
                    });
                }
                _ => {}
            });
    });
}