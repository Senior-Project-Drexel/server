use std::env;
use tokio::io::{self, AsyncReadExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::task;

async fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) | Err(_) => break,
            Ok(size) => {
                dbg!(size);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let port = args[1].parse::<i32>().unwrap();

    let address = format!("127.0.0.1:{}", port);
    println!("Server listening on {}", &address);
    let listener = TcpListener::bind(address).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        task::spawn(handle_client(stream));
    }
}
