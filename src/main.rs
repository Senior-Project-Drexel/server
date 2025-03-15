use std::env;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::task;

mod matrix;
use matrix::Matrix;

fn bit_pack(bytes: &[u8]) -> i32 {
    let packed = ((bytes[0] as i32) << 24)
        | ((bytes[1] as i32) << 16)
        | ((bytes[2] as i32) << 8)
        | (bytes[3] as i32);
    packed
}

async fn client(mut stream: TcpStream) -> io::Result<()> {
    let mut byte_buf = [0; 1024];
    let mut num_buf = vec![];
    let mut matrix_buf = vec![];

    let mut left = 0;

    println!("Client connected");
    loop {
        let size = match stream.read(&mut byte_buf[left..1024 - left]).await {
            Ok(0) | Err(_) => break,
            Ok(size) => size,
        };

        let n = (left + size) / 4;
        let r = (left + size) % 4;

        for i in 0..n {
            num_buf.push(bit_pack(&byte_buf[i * 4..(i + 1) * 4]));
        }

        for i in 0..r {
            byte_buf[i] = byte_buf[n * 4 + i];
        }

        left = r;

        while num_buf.len() > 2 {
            let r = num_buf[0];
            let c = num_buf[1];
            let size = r * c;

            if num_buf.len() < (size + 2) as usize {
                break;
            }

            let mut m = Matrix::new(r, c);
            m.fill(num_buf[2..(size + 2) as usize].into_iter());
            matrix_buf.push(m);
            num_buf.drain(0..(size + 2) as usize);
        }

        if matrix_buf.len() < 2 {
            continue;
        }

        let b = matrix_buf.pop().unwrap();
        let a = matrix_buf.pop().unwrap();
        let c = a * b;
        let shape = c.shape();
        let mut send_buffer = vec![];
        send_buffer.extend_from_slice(&shape.0.to_be_bytes());
        send_buffer.extend_from_slice(&shape.1.to_be_bytes());

        for i in 0..shape.0 {
            for j in 0..shape.1 {
                send_buffer.extend_from_slice(&(c[i as usize][j as usize] as i32).to_be_bytes());
            }
        }

        stream.write(&send_buffer).await.unwrap();
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
