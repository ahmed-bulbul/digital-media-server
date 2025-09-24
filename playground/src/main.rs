use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:8081").await?;
    println!("UDP socket bound to 127.0.0.1:8081");

    let mut buf = [0u8; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Received {} bytes from {}: {:?}", len, addr, &buf[..len]);
        socket.send_to(&buf[..len], &addr).await?;
    }
}
