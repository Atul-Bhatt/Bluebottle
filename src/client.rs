use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use mio::TryRead;

struct Client {
    server_addr: String,
    server_port: u16,
}

impl Client {
    async fn connect(&mut self) -> Result<(), std::io::Error> {
        let mut stream = TcpStream::connect(format!("{}:{}", &self.server_addr, &self.server_port)).await?;
        let message = "Hello, server!";
        stream.write_all(message.as_bytes()).await?;
        let mut buffer = [0; 128];
        let mut bytes_read = 0;
        while bytes_read < buffer.len() {
            bytes_read += stream.try_read(&mut buffer[bytes_read..])?;
        }
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Server response: {}", response);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut client = Client {
        server_addr: String::from("127.0.0.1"),
        server_port: 22,
    };
    client.connect().await?;
    Ok(())
}
