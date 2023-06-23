use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

use log::info;

pub fn start(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?;

    info!("Server listening on {}", address);

    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    info!("New connection from {}", stream.peer_addr()?);

    let reader = BufReader::new(&mut stream);
    let data: Vec<_> = reader
        .lines()
        .take_while(|line| line.as_ref().is_ok_and(|line| !line.is_empty()))
        .map(|line| line.unwrap())
        .collect();

    info!("Received {:#?}", data);

    Ok(())
}
