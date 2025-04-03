use std::env;
use prost::Message;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::task;
use bytes::BytesMut;
use anyhow::Result;
use byteorder::{BigEndian, ByteOrder};

mod matrix;
use matrix::Matrix;

pub mod matrix_proto {
    include!(concat!(env!("OUT_DIR"), "/matrix_proto.rs"));
}
use matrix_proto::MatrixMessage;

trait Decode where Self: Sized {
    async fn decode(stream: &mut TcpStream) -> Result<Self>;
}

impl <T> Decode for T where T: prost::Message + Default {
    async fn decode(stream: &mut TcpStream) -> Result<Self> {
        let mut length_buffer = [0u8; 4];
        let mut message_buffer = BytesMut::with_capacity(1024);

        stream.read_exact(&mut length_buffer).await?;
        let message_length = BigEndian::read_u32(&length_buffer) as usize;
        message_buffer.resize(message_length, 0);

        stream.read_exact(&mut message_buffer).await?;
        Ok(T::decode(message_buffer)?)
    }
}

async fn client(mut stream: TcpStream) -> io::Result<()> {
    println!("Client connected");
    loop {
        let msg_a = match <MatrixMessage as Decode>::decode(&mut stream).await {
            Ok(msg) => msg,
            Err(_) => break
        };
        let mut a = Matrix::new(msg_a.rows, msg_a.cols);
        a.fill(msg_a.data.iter());

        let msg_b = match <MatrixMessage as Decode>::decode(&mut stream).await {
            Ok(msg) => msg,
            Err(_) => break
        };
        let mut b = Matrix::new(msg_b.rows, msg_b.cols);
        b.fill(msg_b.data.iter());

        let c = a * b;
        let mut msg_c = MatrixMessage::default();
        msg_c.rows = c.shape().0;
        msg_c.cols = c.shape().1;
        msg_c.data = c.collect();

        let mut buf = BytesMut::new();
        msg_c.encode(&mut buf).unwrap();
        let len_bytes = (msg_c.encoded_len() as u32).to_be_bytes();
        stream.write_all(&len_bytes).await.unwrap();
        stream.write_all(&buf).await.unwrap();
        println!("Sent result");
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
        task::spawn(client(stream));
    }
}
