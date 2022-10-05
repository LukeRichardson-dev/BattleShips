use tokio::net::TcpSocket;
use std::io;

mod game;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = TcpSocket::new_v6()?;
    let conn = sock.connect("[::1]:8080".parse().unwrap()).await?;

    conn.try_write(b"hello")?;

    Ok(())
}
