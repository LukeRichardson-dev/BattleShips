use tokio::net::{TcpSocket, TcpStream};
use std::{io, net::{SocketAddr}};

mod game;

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
                handle((_sock, address));
            },
            Err(_) => println!("Error"),
        }

    }

    Ok(())
}

async fn handle((sock, address): (TcpStream, SocketAddr)) {

    loop {
        let mut buf = vec![];

        match sock.try_read(&mut buf) {
            Ok(0) => break,
            _ => continue,
        }
    }

}