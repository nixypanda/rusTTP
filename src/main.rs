use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

mod handlers;
mod types;

const BUFFER_SIZE: usize = 4096;

fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut buffer = [0; BUFFER_SIZE];
    let n = stream.peek(&mut buffer)?;

    if n == 0 {
        return Ok(());
    }

    stream.read(&mut buffer)?;

    let response = handlers::respond_with_200()?;

    stream.write_all(&response.as_bytes())?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let stream = stream?;
        handle_client(stream)?;
    }

    Ok(())
}
