use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

mod handlers;
mod parse;
mod types;

const BUFFER_SIZE: usize = 4096;

fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut buffer = [0; BUFFER_SIZE];
    let n = stream.peek(&mut buffer)?;

    if n == 0 {
        return Ok(());
    }
    let raw_request = String::from_utf8_lossy(&buffer[..n]);
    let parsed_request = parse::parse_request(&raw_request)?;

    stream.read(&mut buffer)?;

    let response = if parsed_request.path == "/" {
        handlers::respond_with_200()
    } else if parsed_request.path.starts_with("/echo") {
        handlers::respond_with_path_content(parsed_request)
    } else if parsed_request.path == "/user-agent" {
        handlers::respond_with_user_agent(parsed_request)
    } else {
        handlers::respond_with_404()
    };

    stream.write_all(&response?.as_bytes())?;

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
