// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)]

mod commands;
mod init;

use commands::send_passphrase;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            send_passphrase
        ])
        .setup(|app| {
            let app_handle = app.handle();
            // init::init().expect("failed to initialize");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
