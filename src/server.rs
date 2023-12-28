use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:22").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async
                     move {
                         let mut buffer = [0; 1024];
                         loop {
                             let n = match socket.read(&mut buffer).await {
                                 Ok(n) if n == 0 => return,
                                 Ok(n) => n,
                                 Err(e) => {
                                     eprintln!("Error reading from socket: {}", e);
                                     return;
                                 }
                             };

                             let body = String::from_utf8_lossy(&buffer[..n]);

                             // Basic command parsing
                             let tokens: Vec<&str> = body.split_whitespace().collect();
                             let command = tokens[0];

                             match command {
                                 "USER" => {
                                     // Handler USER command (e.g, authenticate user)
                                     println!("Received USER command");
                                     socket.write_all(b"331 Username OK").await.unwrap();
                                 }
                                 _ => {
                                     println!("Unknown command:{}", command);
                                     socket.write_all(b"500 Unknown command").await.unwrap();
                                 }
                             }
                         }
                     });
    }
}
