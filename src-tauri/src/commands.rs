use std::io::Write;

use base64::{engine::general_purpose, Engine};
use tauri::Manager;

use crate::{
    file_transfer::file_transfer, FileTransferState,
};

#[tauri::command]
pub(crate) fn send_file(
    window: tauri::Window,
    file_path: String,
) -> Result<(), String> {
    println!("send file: {}", file_path);

    file_transfer(&window, &file_path);
    Ok(())
}

#[tauri::command]
pub(crate) fn receive_file(
    window: tauri::Window,
    app_handle: tauri::AppHandle,
    filename: String,
    state: tauri::State<FileTransferState>,
) -> Result<(), String> {
    // println!("receive file chunks: {}", &chunk_count);
    // let filename = String::from_utf8(
    //     filename.split_first().unwrap().1.to_vec(),
    // )
    // .unwrap();
    println!("window: {}", window.label());
    let filename_u8 = general_purpose::STANDARD_NO_PAD
        .decode(filename)
        .unwrap();
    println!("filename_u8: {:?}", filename_u8);
    // if let Some((_, filename)) = filename_u8.split_at(2) {
    let (header, filename) = filename_u8.split_at(3);
    let chunk_count = header[1];
    println!("chunk count: {}", chunk_count);
    let filename_u8_length = header[2];
    let (filename, _) =
        filename.split_at(filename_u8_length as usize);
    let filename =
        String::from_utf8(filename.to_vec()).unwrap();
    println!("filename: {}", filename);
    let mut filename_guard =
        state.filename.try_lock().unwrap();
    *filename_guard = Some(filename);

    state
        .progress
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    // }
    // println!("filename: {}", filename);
    // TODO: save filename to global state

    let listener = window.listen("file_data", move |ev| {
        println!("receiving chunks...");
        let payload = ev.payload().unwrap();
        println!("file data: {:?}", payload);
        let payload = payload
            .trim_matches('"')
            .trim_matches('\\')
            .trim_matches('"');
        println!("file data: {:?}", payload);
        let u8_arr = general_purpose::STANDARD_NO_PAD
            .decode(payload)
            .unwrap();
        if let Some((idx, u8_arr)) = u8_arr.split_first() {
            println!("idx: {}", idx);
            println!("chunk: {:?}", u8_arr);
            let state =
                app_handle.state::<FileTransferState>();
            let mut filename_guard =
                state.filename.try_lock().unwrap();
            let filename = filename_guard.as_ref().unwrap();

            let progress = state.progress.fetch_add(
                1,
                std::sync::atomic::Ordering::SeqCst,
            );

            if progress == *idx {
                // TODO: assemble file
                let files_path =
                    get_file_path(&app_handle).unwrap();
                assemble_file(
                    u8_arr,
                    filename,
                    &files_path,
                )
                .expect("file assemble error");
            }

            if progress == chunk_count {
                println!("file transfer done");
                *filename_guard = None;
                state.progress.store(
                    0,
                    std::sync::atomic::Ordering::SeqCst,
                );

                let mut event_listener_lock = state
                    .event_listener
                    .try_lock()
                    .unwrap();
                if let Some(listener) =
                    event_listener_lock.take()
                {
                    let window = app_handle
                        .get_window("main")
                        .unwrap();
                    window.unlisten(listener);
                }
            }
        };
        // let u8_arr = Vec::<u8>::from(payload);
        // println!("file data: {:?}", u8_arr);
    });

    // TODO: manage listener in global state, so that we can unlisten after file transfer is done
    let mut event_listener_guard =
        state.event_listener.try_lock().unwrap();
    *event_listener_guard = Some(listener);
    // window.unlisten(listener);

    Ok(())
}

fn assemble_file(
    u8_arr: &[u8],
    filename: &str,
    files_path: &std::path::PathBuf,
) -> Result<(), String> {
    println!("chunk: {:?}", u8_arr);
    let file_path = files_path.join(filename);

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();
    file.write_all(u8_arr).unwrap();

    Ok(())
}

#[tauri::command]
pub(crate) fn open_file_folder(
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    use std::process::Command;

    let files_path = get_file_path(&app_handle)
        .map_err(|e| e.to_string())?;
    let path_str = files_path.to_str().unwrap();

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", path_str]) // The comma after select is not a typo
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        if path_str.contains(',') {
            // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
            let new_path = match std::fs::metadata(path_str)
                .unwrap()
                .is_dir()
            {
                true => path_str.to_owned(),
                false => {
                    let mut path2 =
                        std::path::PathBuf::from(path_str);
                    path2.pop();
                    path2
                        .into_os_string()
                        .into_string()
                        .unwrap()
                }
            };
            Command::new("xdg-open")
                .arg(&new_path)
                .spawn()
                .map_err(|e| e.to_string())?;
        } else {
            Command::new("dbus-send")
                .args([
                    "--session",
                    "--dest=org.freedesktop.FileManager1",
                    "--type=method_call",
                    "/org/freedesktop/FileManager1",
                    "org.freedesktop.FileManager1.ShowItems",
                    format!("array:string:\"file://{path_str}\"").as_str(),
                    "string:\"\"",
                ])
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(["-R", path_str])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn get_file_path(
    app_handle: &tauri::AppHandle,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>>
{
    let path = app_handle
        .path_resolver()
        .app_data_dir()
        // .resolve_resource("./files")
        .unwrap()
        .join("files");
    println!("path: {:?}", path);
    if let Err(e) = std::fs::File::open(&path) {
        println!("file open error: {}", e);
        std::fs::create_dir_all(&path)?;
    }

    Ok(path)
}
