use std::{fs, process};
use plume_core::{config::{self, Friend}, encryption};

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
            let author_name= &config.me.username;

            let author_ed: String = String::from_utf8(fs::read(&config.me.public_ed_path).expect("Unable to access user's public key")).expect("Unable to read user's public key'");
            let author_private_ed: String = String::from_utf8(fs::read(&config.me.private_ed_path).expect("Unable to access user's private key")).expect("Unable to read user's private key'");


            // Generate the request
            let [request_packet, _, author_x_priv] = plume_core::relay_interaction::request_friend(&target_ed, &author_ed, author_name, &author_private_ed);

            // Generate new friend and add it to config
            let friend: Friend = Friend {
                username: "".to_string(),
                public_ed: target_ed.clone(),
                profile_picture: "".to_string(),
                shared_key: "".to_string(),
                last_sync: "".to_string(),
                private_x: author_x_priv
            };

            config.friends.insert(target_ed, friend);

            // Save the config
            config::update_config(&config);

            display_info!("Sending friend request");

            Some(request_packet)
        },
        "/accept_friend" => {
            if args.len() != 2 {
                display_error!("Invalid commands arguments, please provide a single request_id");
            }

            // check for a friend request with the given id
            let mut config = plume_core::config::get_config();
            if let Some(friend_request) = config.friend_requests.remove(args[1]) {
                // Generate packet to send to relay & keys
                let user_public_ed= String::from_utf8(fs::read(config.me.public_ed_path).expect("Unable to access the local signign key")).expect("Invalid key file stored");
                let user_private_ed= String::from_utf8(fs::read(config.me.private_ed_path).expect("Unable to access the local signign key")).expect("Invalid key file stored");
                let [packet, _, author_private_x] = plume_core::relay_interaction::request_friend(&friend_request.friend_public_ed, &user_public_ed, &config.me.username,&user_private_ed);
                
                // Generate the new friend data
                let shared_key = encryption::generate_shared_key(&author_private_x, &friend_request.friend_public_x);
                let friend = Friend {
                    public_ed: friend_request.friend_public_ed.clone(),
                    private_x: author_private_x,
                    shared_key,
                    username: friend_request.username.clone(),
                    profile_picture: friend_request.profile_picture.clone(),
                    last_sync: "".to_string()
                };

                // Store the new friend data
                config.friends.insert(friend_request.friend_public_ed, friend);


                return Some(packet);
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
