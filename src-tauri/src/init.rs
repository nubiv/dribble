use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    tauri::async_runtime::spawn(async move {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
            .await
            .expect("Failed to bind the port");

        loop {
            let (mut socket, _) = listener.accept().await.expect("Failed to listen on port");
            println!("Accepted connection from {:?}", socket.peer_addr().unwrap());

            tauri::async_runtime::spawn(async move {
                let mut buf = [0; 1024];

                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => {
                            println!("read {} bytes", n);
                            n
                        }
                        Err(e) => {
                            eprintln!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    });

    Ok(())
}
