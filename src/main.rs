use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

use handlers::Handler;

mod file;
mod handlers;
mod parse;
mod types;

const BUFFER_SIZE: usize = 4096;

fn handle_client(mut stream: TcpStream, handler: Arc<Handler>) -> anyhow::Result<()> {
    let mut buffer = [0; BUFFER_SIZE];
    let n = stream.peek(&mut buffer)?;

    if n == 0 {
        return Ok(());
    }
    let raw_request = String::from_utf8_lossy(&buffer[..n]);
    let parsed_request = parse::parse_request(&raw_request)?;

    stream.read(&mut buffer)?;

    let response = if parsed_request.path == "/" {
        handler.respond_with_200()
    } else if parsed_request.path.starts_with("/echo") {
        handler.respond_with_path_content(parsed_request)
    } else if parsed_request.path == "/user-agent" {
        handler.respond_with_user_agent(parsed_request)
    } else if parsed_request.path.starts_with("/files") {
        if parsed_request.method == "GET" {
            handler.respond_with_file(parsed_request)
        } else {
            handler.respond_with_404()
        }
    } else {
        handler.respond_with_404()
    };

    stream.write_all(&response?.as_bytes())?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let args: Vec<String> = env::args().collect();

    let env_dir = if args.len() < 3 {
        ".".to_string()
    } else {
        assert_eq!(args[1], "--directory");
        args[2].clone()
    };

    let handler = Arc::new(Handler::new(env_dir));

    for stream in listener.incoming() {
        let stream = stream?;
        let h = handler.clone();
        thread::spawn(move || -> anyhow::Result<()> {
            handle_client(stream, h)?;
            Ok(())
        });
    }

    Ok(())
}
