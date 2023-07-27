use std::io::{BufReader, Read};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct FileSignalRes {
    signal: u8,
    chunk: Option<u8>,
}

pub(crate) fn file_transfer(
    window: &tauri::Window,
    file_path: &str,
) {
    let file = match std::fs::File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("file open error: {}", e);
            return;
        }
    };
    let bytes = BufReader::new(file).bytes();

    for byte in bytes {
        let byte = byte.unwrap();
        window
            .emit(
                "file",
                &FileSignalRes {
                    signal: byte,
                    chunk: None,
                },
            )
            .unwrap();
    }

    window
        .emit(
            "file_signal",
            &FileSignalRes {
                signal: 1,
                chunk: None,
            },
        )
        .unwrap();
}
