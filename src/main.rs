use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

#[tokio::main]
async fn main(){
    let listener= TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("Server is running on port 127.0.0.1:8000");

    let (socket,addr)=listener.accept().await.unwrap();
    println!("New connection from: {}",addr);

    let mut reader=tokio::io::BufReader::new(socket).lines();

    while let Some(line)=reader.next_line().await.unwrap(){
        println!("Received: {}",line);
    }

    println!("Connection closed");

}