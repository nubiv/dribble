// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)]

mod commands;
mod file_transfer;

use commands::{
    __cmd__open_file_folder, __cmd__send_file,
    open_file_folder, send_file,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            send_file,
            open_file_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
