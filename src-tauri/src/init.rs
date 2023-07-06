pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // tauri::async_runtime::spawn(async move {
    //     let listener =
    //         tokio::net::TcpListener::bind("127.0.0.1:8080")
    //             .await
    //             .expect("Failed to bind the port");

    //     loop {
    //         let (mut socket, _) = listener
    //             .accept()
    //             .await
    //             .expect("Failed to listen on port");
    //         println!(
    //             "Accepted connection from {:?}",
    //             socket.peer_addr().unwrap()
    //         );
    //     }
    // });

    Ok(())
}
