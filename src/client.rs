use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::IpAddr;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    name_server::{GenericConnection, TokioRuntime},
    AsyncResolver,
};
use std::io;
use std::env;

struct Client {
    server_addr: String,
    server_port: u16,
}

impl Client {
    async fn connect(&mut self) -> Result<(), std::io::Error> {
        let mut stream = TcpStream::connect(format!("{}:{}", &self.server_addr, &self.server_port)).await?;
        loop {
            let mut message = String::new();
            io::stdin().read_line(&mut message).expect("Cannot read a line!");
            stream.write_all(message.as_bytes()).await?;
            let mut buffer = [0; 128];
            let mut bytes_read = 0;
            while bytes_read < buffer.len() {
                bytes_read += stream.try_read(&mut buffer[bytes_read..])?;
            }
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Server response: {}", response);
            if message == "quit" {
                break;
            }
        }
    Ok(())
    }
}

async fn get_address_from_name(server_name: String) -> String {
    let resolver = AsyncResolver::from_system_conf(ResolverConfig::default(), ResolverOpts::default())
        .unwrap();
    
    let lookup_future = resolver.lookup_ip(server_name); // Note the trailing dot

    let lookup_result = async {
        lookup_future.await.expect("Failed to lookup address")
    };

    let (addresses, _) = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(lookup_result)
        .expect("Failed to get addresses");

    let server_address: String;
    for address in addresses {
        match address {
            IpAddr::V4(ip) => server_address = ip.to_string(),
            _ => (),
        }
    }
    server_address
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = env::args().collect();
    
    server_address = get_address_from_name(args[1]);

    let mut client = Client {
        server_addr: String::from(args[1]),
        server_port: 22,
    };
    client.connect().await?;
    Ok(())
}
