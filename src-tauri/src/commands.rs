#[tauri::command]
pub fn send_passphrase(
    passphrase: &str,
) -> Result<(), String> {
    println!("Passphrase: {}", passphrase);

    // let entry_guard =
    //     connection_state.entry.lock().unwrap();
    // let entry = entry_guard.as_ref();

    // match entry {
    //     Some(passphrase) => {
    //         println!("Entry: {:?}", passphrase);
    //     }
    //     None => {
    //         println!("Entry: None");
    //     }
    // };

    Ok(())
}
