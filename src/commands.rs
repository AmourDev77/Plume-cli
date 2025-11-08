use std::{fs, process};
use plume_core::{config::{self, Friend}, packets::relay::friend_request::{self, AuthorInfo}};

use crate::colors;

pub fn command_list() -> Vec<String> {
    ["/exit", "/add_friend", "/help", "/request_friend"]
        .iter().map(|cmd| cmd.to_string()).collect()
}

pub fn execute_command(command: &str, args: Vec<&str>) -> Option<String> {
    match command.split(' ').next().unwrap() {
        "/exit" => {
            process::exit(0);
        },
        "/help" => {
            command_list().iter().for_each(|cmd|  {
                print!("{} | ", cmd)
            });
            None
        }
        "/request_friend" => {
            println!("{:?}",args);
            if args.len() < 2 {
                println!("{}", colors::error("A path to a public key must be given to use this command"));
                return None
            }

            // Retrieve the key given by the user
            if !fs::exists(args[1]).expect("Unable to access the given path") {
                println!("{}", colors::error("Invlaid path provided"));
                return None
            }

            let target_ed = String::from_utf8(fs::read(args[1]).expect("Error opening the given path")).expect("Error reading key information");


            // Retrieve the users details 
            let mut config = plume_core::config::get_config();

            let author_public_ed: String = String::from_utf8(fs::read(&config.me.public_ed_path).expect("Unable to access user's public key")).expect("Unable to read user's public key'");
            let author_private_ed: String = String::from_utf8(fs::read(&config.me.private_ed_path).expect("Unable to access user's private key")).expect("Unable to read user's private key'");

            // generate a shared key for the friend reqest

            let user_information = AuthorInfo {
                author_name: &config.me.username,
                author_private_ed: &author_private_ed,
                author_public_ed: &author_public_ed,
                author_picture: &config.me.profile_picture
            };

            // Generate the request
            let packet = friend_request::generate_friend_request_packet(&target_ed, user_information, "");// TODO: Generate the shared key
            
            if let Err(error) = packet {
                eprintln!("An error occured : {error}");
                return None
            }

            // Generate new friend and add it to config
            let friend: Friend = Friend {
                username: "".to_string(),
                public_ed: target_ed.clone(),
                profile_picture: "".to_string(),
                shared_key: "".to_string(),
                last_sync: "".to_string(),
                private_x: "".to_string() // TODO: Generate the shared key
            };

            config.friends.insert(target_ed, friend);

            // Save the config
            config::update_config(&config);

            display_info!("Sending friend request");

            Some(packet.unwrap()) // WARNING: there is probably a better way to do, to be checked
        },
        "/accept_friend" => {
            if args.len() != 2 {
                display_error!("Invalid commands arguments, please provide a single request_id");
            }

            // check for a friend request with the given id
            let mut config = plume_core::config::get_config();
            if let Some(friend_request) = config.friend_requests.remove(args[1]) {
                // // Generate packet to send to relay & keys
                // let user_public_ed= String::from_utf8(fs::read(config.me.public_ed_path).expect("Unable to access the local signign key")).expect("Invalid key file stored");
                // let user_private_ed= String::from_utf8(fs::read(config.me.private_ed_path).expect("Unable to access the local signign key")).expect("Invalid key file stored");
                //
                // let [packet, _, author_private_x] = relay::request_friend(&friend_request.friend_public_ed, &user_public_ed, &config.me.username,&user_private_ed);
                // 
                // // Generate the new friend data
                // let shared_key = encryption::keys::generate_shared_key(&author_private_x, &friend_request.friend_public_x).expect("Error generating key"); 
                // let friend = Friend {
                //     public_ed: friend_request.friend_public_ed.clone(),
                //     private_x: author_private_x,
                //     shared_key,
                //     username: friend_request.username.clone(),
                //     profile_picture: friend_request.profile_picture.clone(),
                //     last_sync: "".to_string()
                // };
                //
                //
                // TODO: Rewrite the process
                //
                // Store the new friend data
                // config.friends.insert(friend_request.friend_public_ed, friend);


                return Some("".to_string());
            } else {
                display_error!("No friend request found with this is : {}", args[1]);
                return None;
            };
        },
        &_ => {
            println!("Command not implemented yet");
            None
        }
    }
}
