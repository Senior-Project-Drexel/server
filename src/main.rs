use anyhow::Result;
use byteorder::{BigEndian, ByteOrder};
use bytes::BytesMut;
use prost::Message;
use std::env;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::task;

mod matrix;
use matrix::Matrix;

pub mod matrix_proto {
    include!(concat!(env!("OUT_DIR"), "/matrix_proto.rs"));
}
use matrix_proto::{MatrixRequest, MatrixResponse};

trait Decode
where
    Self: Sized,
{
    async fn decode(stream: &mut TcpStream) -> Result<Self>;
}

impl<T> Decode for T
where
    T: prost::Message + Default,
{
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
        let request = match <MatrixRequest as Decode>::decode(&mut stream).await {
            Ok(req) => req,
            Err(_) => break,
        };

        let matrix1 = request.matrix1.unwrap();
        let matrix2 = request.matrix2.unwrap();

        let mut a = Matrix::new(matrix1.rows, matrix1.cols);
        a.fill(matrix1.data.iter());

        let mut b = Matrix::new(matrix2.rows, matrix2.cols);
        b.fill(matrix2.data.iter());

        let c = a * b;

        let mut response = MatrixResponse::default();
        response.id = request.id;
        let mut result_matrix = matrix_proto::Matrix::default();
        result_matrix.rows = c.shape().0;
        result_matrix.cols = c.shape().1;
        result_matrix.data = c.collect();
        response.matrix = Some(result_matrix);

        let mut buf = BytesMut::new();
        response.encode(&mut buf).unwrap();
        let len_bytes = (response.encoded_len() as u32).to_be_bytes();
        stream.write_all(&len_bytes).await.unwrap();
        stream.write_all(&buf).await.unwrap();
        println!("Sent result for request ID: {}", request.id);
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
