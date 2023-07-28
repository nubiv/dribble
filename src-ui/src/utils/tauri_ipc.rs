use tauri_sys::{event, tauri};
use wasm_bindgen::prelude::*;

#[derive(serde::Serialize)]
pub(crate) struct OpenFileFolderCmdArgs();

#[derive(serde::Serialize)]
pub(crate) struct SendFileCmdArgs {
    #[serde(rename(serialize = "filePath"))]
    file_path: String,
}

#[derive(serde::Serialize)]
pub(crate) struct ReceiveFileCmdArgs {
    filename: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct FileTransferRes {
    signal: u8,
    chunk: Option<u8>,
}

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
//     pub(crate) async fn invoke(
//         cmd: &str,
//         args: JsValue,
//     ) -> JsValue;
// }

pub(crate) async fn invoke_open_file_folder(
) -> Result<(), String> {
    if let Err(e) = tauri::invoke::<_, ()>(
        "open_file_folder",
        &OpenFileFolderCmdArgs {},
    )
    .await
    {
        return Err(e.to_string());
    };

    Ok(())
}

pub(crate) async fn invoke_receive_file(
    filename: String,
) -> Result<(), String> {
    if let Err(e) = tauri::invoke::<_, ()>(
        "receive_file",
        &ReceiveFileCmdArgs { filename },
    )
    .await
    {
        return Err(e.to_string());
    };

    Ok(())
}

// pub(crate) async fn listen_on_file_transfer_event(
// ) -> Result<(), String> {
//     let mut signals =
//         event::listen::<FileTransferRes>("file_signal")
//             .await
//             .unwrap();

//     while let Some(signal) = signals.next().await {
//         log!("file signal: {:?}", signal.payload);
//     }

//     Ok(())
// }

pub(crate) async fn emit_file_data(
    data: String,
) -> Result<(), String> {
    if let Err(e) = event::emit("file_data", &data).await {
        return Err(e.to_string());
    };

    Ok(())
}
