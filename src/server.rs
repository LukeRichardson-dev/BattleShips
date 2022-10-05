use tokio::net::{TcpSocket, TcpStream};
use std::{io, net::{SocketAddr}, str::from_utf8};

mod game;
mod pool;
use game::Commands;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "[::1]:8080".parse().unwrap();
    
    let sock = TcpSocket::new_v6()?;
    sock.bind(addr)?;

    let listener = sock.listen(128)?;

    // let threads: = vec![];

    loop {

        match listener.accept().await {
            Ok((_sock, address)) => {
                println!("Got connection from {}", address);

                tokio::spawn(handle((_sock, address)));
            },
            Err(_) => println!("Error"),
        }

    }
}

async fn handle((sock, address): (TcpStream, SocketAddr)) {

    sock.readable().await.unwrap();

    loop {

        let mut buf = [0; 4096];

        let n = match sock.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("{:?}", &buf[..n]);
        let cmd: Commands = serde_json::from_slice(&buf[..n]).unwrap();
    
        match cmd {

            Commands::Strike { x, y, user } => {

                println!(
                    "[User <{}\\>({})] struck the position ({}, {})", 
                    user, 
                    address, 
                    x, y,
                );

            },

            Commands::Sync => {

            }

            _ => {},
            
        }

    }

}