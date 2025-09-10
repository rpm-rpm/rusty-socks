use std::io;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// SOCKS5 protocol constants
const SOCKS5_VERSION: u8 = 0x05;
const NO_AUTHENTICATION: u8 = 0x00;
const CONNECT: u8 = 0x01;
const IPV4: u8 = 0x01;
const DOMAIN: u8 = 0x03;
const IPV6: u8 = 0x04;
const SUCCESS: u8 = 0x00;
const CONNECTION_REFUSED: u8 = 0x05;

pub async fn handle_client(mut socket: TcpStream) -> io::Result<()> {
    // Step 1: Read client greeting
    let mut greeting = [0u8; 2];
    socket.read_exact(&mut greeting).await?;

    if greeting[0] != SOCKS5_VERSION {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid SOCKS version"));
    }

    let num_methods = greeting[1] as usize;
    let mut methods = vec![0u8; num_methods];
    socket.read_exact(&mut methods).await?;

    // Check if no authentication is supported
    let method = if methods.contains(&NO_AUTHENTICATION) {
        NO_AUTHENTICATION
    } else {
        // For simplicity, we'll reject other authentication methods
        0xFF
    };

    // Step 2: Send server greeting response
    let response = [SOCKS5_VERSION, method];
    socket.write_all(&response).await?;

    if method == 0xFF {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "No acceptable authentication method"));
    }

    // Step 3: Read connection request
    let mut request_header = [0u8; 4];
    socket.read_exact(&mut request_header).await?;

    if request_header[0] != SOCKS5_VERSION {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid SOCKS version"));
    }

    if request_header[1] != CONNECT {
        // For simplicity, only support CONNECT command
        send_reply(&mut socket, CONNECTION_REFUSED, &[0, 0, 0, 0, 0, 0]).await?;
        return Ok(());
    }

    // Skip RSV byte
    let addr_type = request_header[3];

    // Step 4: Parse destination address
    let dest_addr = match addr_type {
        IPV4 => {
            let mut addr = [0u8; 4];
            socket.read_exact(&mut addr).await?;
            let mut port = [0u8; 2];
            socket.read_exact(&mut port).await?;
            format!("{}.{}.{}.{}:{}", addr[0], addr[1], addr[2], addr[3], u16::from_be_bytes(port))
        }
        DOMAIN => {
            let mut len = [0u8; 1];
            socket.read_exact(&mut len).await?;
            let len = len[0] as usize;
            let mut domain = vec![0u8; len];
            socket.read_exact(&mut domain).await?;
            let mut port = [0u8; 2];
            socket.read_exact(&mut port).await?;
            format!("{}:{}", String::from_utf8_lossy(&domain), u16::from_be_bytes(port))
        }
        IPV6 => {
            let mut addr = [0u8; 16];
            socket.read_exact(&mut addr).await?;
            let mut port = [0u8; 2];
            socket.read_exact(&mut port).await?;
            // For simplicity, convert IPv6 to string representation
            let ipv6_str = format!("{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
                u16::from_be_bytes([addr[0], addr[1]]),
                u16::from_be_bytes([addr[2], addr[3]]),
                u16::from_be_bytes([addr[4], addr[5]]),
                u16::from_be_bytes([addr[6], addr[7]]),
                u16::from_be_bytes([addr[8], addr[9]]),
                u16::from_be_bytes([addr[10], addr[11]]),
                u16::from_be_bytes([addr[12], addr[13]]),
                u16::from_be_bytes([addr[14], addr[15]]));
            format!("[{}]:{}", ipv6_str, u16::from_be_bytes(port))
        }
        _ => {
            send_reply(&mut socket, CONNECTION_REFUSED, &[0, 0, 0, 0, 0, 0]).await?;
            return Ok(());
        }
    };

    // Step 5: Connect to destination
    match TcpStream::connect(&dest_addr).await {
        Ok(dest_socket) => {
            // Send success reply
            send_reply(&mut socket, SUCCESS, &[0, 0, 0, 0, 0, 0]).await?;

            // Step 6: Forward data between client and destination
            forward_data(socket, dest_socket).await?;
        }
        Err(_) => {
            send_reply(&mut socket, CONNECTION_REFUSED, &[0, 0, 0, 0, 0, 0]).await?;
        }
    }

    Ok(())
}

async fn send_reply(socket: &mut TcpStream, reply: u8, bind_addr: &[u8]) -> io::Result<()> {
    let mut response = vec![SOCKS5_VERSION, reply, 0, IPv4];
    response.extend_from_slice(bind_addr);
    socket.write_all(&response).await
}

async fn forward_data(mut client: TcpStream, mut dest: TcpStream) -> io::Result<()> {
    use tokio::io::copy_bidirectional;
    copy_bidirectional(&mut client, &mut dest).await?;
    Ok(())
}
