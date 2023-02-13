use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};

#[tokio::main]
async fn main() {
    // step1: create a tcp echo server
    // this server will wait for client to connect
    // once the client is connected, it will take any message from the client
    // and echo it back to the client
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    
    let (mut socket, _addr) = listener.accept().await.unwrap();

    let mut buffer = [0u8; 1024];

    let bytes_read = socket.read(&mut buffer).await.unwrap();

    socket.write_all(&buffer[0..bytes_read]).await.unwrap();
}
