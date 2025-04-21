use std::io;

use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

mod commands;
mod colors;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let url = "ws://localhost:8081";

    plume_core::init();

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

        input = input[..input.len()-1].to_string();

        println!("Input is : '{}'", input);
        
        // Command recognition
        if let Some(command) = commands::command_list().into_iter().find(|cmd| cmd == &input) {
            commands::execute_command(&command);
            continue;
        };

        // If no command recognized then sending the message
        let message = Message::text(input);

        write.send(message).await.expect("Failde to send message to server");
    }
}

fn add_friend() {
    // First generate and display the keys :
    let (secret, public_key ) = plume_core::encryption::generate_keys();

    println!("Transmit this private key to add a friend : {}", String::from_utf8(public_key.to_bytes().to_vec()).expect("Unable to get public key"))

    // next input will be dedicated to input x25519 key
}
