use tokio::net::TcpSocket;
use std::io;

mod game;
use game::Commands;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = TcpSocket::new_v6()?;
    let conn = sock.connect("[::1]:8080".parse().unwrap()).await?;

    let command = Commands::Strike {
            x: 1, y: 2, user: 200,
        };

    let cbytes = serde_json::to_vec(&command).unwrap();
    println!("{:?}", cbytes);

    conn.writable().await?;
    conn.try_write(&cbytes)?;

    Ok(())
}
