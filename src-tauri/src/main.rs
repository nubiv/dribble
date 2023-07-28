// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)]

mod commands;
mod file_transfer;

use std::sync::atomic::AtomicU8;

use commands::{
    __cmd__open_file_folder, __cmd__receive_file,
    __cmd__send_file, open_file_folder, receive_file,
    send_file,
};
use futures::lock::Mutex;
use tauri::Manager;

#[derive(Default)]
struct FileTransferState {
    filename: Mutex<Option<String>>,
    progress: AtomicU8,
    event_listener: Mutex<Option<tauri::EventHandler>>,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            send_file,
            receive_file,
            open_file_folder
        ])
        .setup(|app| {
            app.manage(FileTransferState::default());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
