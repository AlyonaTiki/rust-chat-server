use std::io::BufReader;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener};
use tokio::{select, spawn};
use tokio::sync::broadcast;

#[tokio:main]
async fn main() {
    let listener : TcpListener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        spawn(async move
        {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                select! {
                result = reader.read_line(&mut line) =>{
                    if result.unwrap() == 0 {
                        break;
                    }

                    tx.send(line.clone(), addr).unwrap();
                    line.clear();
                }
                result = rx.recv() => {
                    let (msg, other_addr) = result.unwrap();

                    if addr != other_addr {
                        writer.write_all(msg.as_bytes()).await.unwrap();
                    }
                }
            }
            }
        });
    }
}
