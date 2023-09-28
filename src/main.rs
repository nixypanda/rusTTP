use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {
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
