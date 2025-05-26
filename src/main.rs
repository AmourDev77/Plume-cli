use std::{fs, io::BufReader};
use std::{env, io};
use std::fs::File;

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

    // Then Check for config, if user does not have key then propose to generate one or insert an existing one
    let config_path = env::var("PLUME_CONFIG").expect("Config env var not set");
    let config_file = File::open(format!("{}/configs.json", config_path)).expect("Eror opening config file");
    let reader = BufReader::new(config_file);

    let mut configs: plume_core::config::Config = serde_json::from_reader(reader).expect("Unable to convert this file to json");

    let mut signing_key = String::new();
    let mut public_key = String::new();

    if configs.me.public_ed_path.len() == 0 {
        // Then generate public keys
        let (private_ed, public_ed) = plume_core::encryption::generate_ed_keys();
        println!("Private : {}", private_ed);
        println!("Public : {}", public_ed);
        signing_key = private_ed;
        public_key = public_ed;
    }

    match fs::read(&configs.me.public_ed_path) {
        Ok(file) => {
            let public_ed = String::from_utf8(file).expect("Unable to transform file to string");
            println!("Your public key is : \n{}", public_ed);
            signing_key = String::from_utf8(fs::read(&configs.me.private_ed_path).expect("Invalid file storing signing key")).expect("Invalid key stored in file");
            public_key = public_ed;
        },
        Err(err) => {
            println!("Key path : {}", configs.me.public_ed_path);
            println!("{}", err);
            println!("Unable to locate previously generated key, setting up a new one ... ");

            let (private_ed, public_ed) = plume_core::encryption::generate_ed_keys();
            // write the keys to files
            fs::write(format!("{}/keys/private_ed.pem", config_path), &private_ed).expect("Unable to write key to file");
            fs::write(format!("{}/keys/public_ed.pem", config_path), &public_ed).expect("Unable to write key to file");


            println!("Wrote keys to files");

            configs.me.public_ed_path = format!("{}/keys/public_ed.pem", config_path);
            configs.me.private_ed_path = format!("{}/keys/private_ed.pem", config_path);

            println!("{:?}", configs);

            let json = serde_json::json!(configs);
            fs::write(format!("{}/configs.json", config_path),  serde_json::to_vec(&json).expect("Unable to transform string to json")).expect("Unable to write config file");

            println!("Config file updated");

            println!("\n\n Your public key is : \n{}", public_ed);

            signing_key = private_ed;
            public_key = public_ed;
        }
    }

    println!("Connecting to : {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect to ws server");
    println!("Connected to server !");

    let (mut write, mut read) = ws_stream.split();

    // ------ Login ------
    let signed_message = plume_core::encryption::sign_packet(format!("login__{}", public_key), &signing_key);
    let message = Message::text(signed_message);

    write.send(message).await.expect("Failde to send message to server");
    // ------ Login ------

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

        // Command recognition
        if let Some(command) = commands::command_list().into_iter().find(|cmd| cmd == &input) {
            commands::execute_command(&command);
            continue;
        };

        // If no command recognized then sending the message
        // Before seding: signing 

        let signed_message = plume_core::encryption::sign_packet(format!("message__{}__{}__{}", public_key, "future_target" , input), &signing_key);
        let message = Message::text(signed_message);

        write.send(message).await.expect("Failde to send message to server");
    }
}
