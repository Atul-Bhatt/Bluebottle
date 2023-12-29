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
                                     socket.write_all(b"331 Username OK\r\n").await.unwrap();
                                 }
                                 "PASS" => {
                                     // Verify Password
                                     println!("Received PASS command");
                                     socket.write_all(b"230 User logged in\r\n").await.unwrap();
                                 }
                                 "CWD" => {
                                     // Change working directory
                                     println!("Received CWD command");
                                     socket.write_all(b"250 Working directory changed\r\n").await.unwrap();
                                 }
                                 "RETR" => {
                                     // Retrieve the copy of a file
                                     println!("Received RETR command");
                                     socket.write_all(b"250 Retrieving file\r\n").await.unwrap();
                                 }
                                 "PORT" => {
                                     // Address and port to which server should connect
                                     println!("Received PORT command");
                                     socket.write_all(b"250 Connecting to port\n\r").await.unwrap();
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
