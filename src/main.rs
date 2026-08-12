use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

#[tokio::main]
async fn main(){
    let listener= TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("Server is running on port 127.0.0.1:8000");

    let writer =tokio::spawn({
        async move{
            let mut client=tokio::net::TcpStream::connect("127.0.0.1:8000").await.unwrap();
            
            let messages= tokio::io::BufReader::new(tokio::io::stdin());
            let mut lines=messages.lines();

            loop{
                if let Some(line)=lines.next_line().await.unwrap(){
                    client.write_all(line.as_bytes()).await.unwrap();
                    client.write_all(b"\n").await.unwrap();
                }else{
                    break;
                }
            }

        }
    });
    let (socket,addr)=listener.accept().await.unwrap();
    println!("New connection from: {}",addr);

    let mut reader=tokio::io::BufReader::new(socket).lines();

    while let Some(line)=reader.next_line().await.unwrap(){
        println!("Received: {}",line);
    }

    println!("Connection closed");

    tokio::select!{
        _=writer=>{},
        _=tokio::signal::ctrl_c()=>{},
    }

}