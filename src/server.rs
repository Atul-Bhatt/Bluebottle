use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3242").await?;

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

                             socket.write_all(&buffer[..n]).await.unwrap();
                         }
                     });
    }
}
