use std::io;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    let url = "ws://localhost:8081";

    println!("Connecting to : {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect to ws server");
    println!("Connected to server !");

    let (mut write, mut read) = ws_stream.split();

    // Spawning the sender in a background task, not
    tokio::spawn(async move{
        loop {
            if let Some(message) = read.next().await {
                let message = message.expect("Failed to read msg");
                println!("\x1b[34m{}\x1b[0m", message);
            }
        }
    });

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let message = Message::text(input);

        write.send(message).await.expect("Failde to send message to server");
    }
}
