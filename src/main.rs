use std::io;
use tokio::net::TcpListener;

mod socks5;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:1080";
    let listener = TcpListener::bind(addr).await?;
    println!("SOCKS5 proxy listening on {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = socks5::handle_client(socket).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}
