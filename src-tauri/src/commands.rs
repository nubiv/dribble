use std::io::Write;

use crate::file_transfer::file_transfer;

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
    filename: Vec<u8>,
) -> Result<(), String> {
    // println!("receive file chunks: {}", &chunk_count);
    let filename = String::from_utf8(
        filename.split_first().unwrap().1.to_vec(),
    )
    .unwrap();
    println!("filename: {}", filename);
    // TODO: save filename to global state

    let listener = window.listen("file_data", move |ev| {
        let payload = ev.payload().unwrap();
        let u8_arr = Vec::<u8>::from(payload);
        // println!("file data: {:?}", u8_arr);

        // TODO: assemble file
        assemble_file(u8_arr).unwrap();
    });

    // TODO: manage listener in global state, so that we can unlisten after file transfer is done
    // window.unlisten(listener);

    Ok(())
}

fn assemble_file(u8_arr: Vec<u8>) -> Result<(), String> {
    println!("chunk: {:?}", u8_arr);

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("test.txt")
        .unwrap();
    file.write_all(&u8_arr).unwrap();

    Ok(())
}

#[tauri::command]
pub(crate) fn open_file_folder(
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    use std::process::Command;

    let models_path = get_file_path(&app_handle)
        .map_err(|e| e.to_string())?;
    let path_str = models_path.to_str().unwrap();

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
