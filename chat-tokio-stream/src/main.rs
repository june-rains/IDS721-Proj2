use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}};

#[tokio::main]
async fn main() {
    // step1: create a tcp echo server
    // this server will wait for client to connect
    // once the client is connected, it will take any message from the client
    // and echo it back to the client
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    loop{
        let (mut socket, _addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let(reader, mut writer) = socket.split();
    
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
    
            loop{
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    break;
                }
                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
    
}
