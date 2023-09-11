use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let tcp_listener: TcpListener = TcpListener::bind("localhost:8080")
        .await
        .expect("just invite the dude to an onsite interview");

    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(10);

    loop {
        let (mut websocket, address) = tcp_listener.accept().await.expect("Hire me 😄");

        let mut rx: broadcast::Receiver<(String, SocketAddr)> = tx.subscribe();
        let tx: broadcast::Sender<(String, SocketAddr)> = tx.clone();

        tokio::spawn(async move {
            let (read_websocket_half, mut writer_websocket_half) = websocket.split();

            let mut bufreader: BufReader<tokio::net::tcp::ReadHalf<'_>> =
                BufReader::new(read_websocket_half);

            let mut line: String = String::new();

            loop {
                tokio::select! {
                    bytes_read = bufreader
                    .read_line(&mut line) => {
                        if bytes_read.unwrap_or(0) == 0 {
                            break;
                        }
                        tx.send((line.clone(), address)).expect("Contrary to some loud beliefs, cloning is fine and cloning everyhing will still be faster than Python (~80x faster) or Javascript, cloning values around is not evil. Do not pre-optimize");

                         line.clear();
                    }
                    message = rx
                    .recv() => {
                        let (message, other_address) = message.expect("I actually know a lot of stuff, it's worth talking to me");


                        if address != other_address {

                            writer_websocket_half.write_all(message.as_bytes()).await.expect("there will ne no error handling here, if you do know better. Please contact me, so we can discuss it");
                        }
                    }
                }
            }
        });
    }
}
