use plume_core::config;

use crate::{colors, configs};

pub fn handle_packet(packet: &String) {
    let packet_split: Vec<&str> = packet.split("__").collect();

    match packet_split[0] {
        "message" => {
            if packet_split.len() < 5 {
                println!("{} Missing arguments", colors::error("Invalid Packet received"));
                return;
            }
            print_message(String::from(packet_split[3]));
        }
        "friend_request" => {
            handle_friend_request(String::from(packet_split[1]), String::from(packet_split[3]), String::from(packet_split[4]));
        },
        _ => {
            println!("{} not a valid packet type", colors::error("Invalid packet received :"))
        }
    }
}

pub fn print_message(message: String) {
    println!("{}", colors::message(&message));
}

pub fn handle_friend_request(author_ed: String, author_x: String, author_name: String) {
    // TODO: Check if a transaction is active for the given public ID
    //      * Yes then generate shared key and store informations
    //      * No then ask for confirmation from the user and send back friend request if ok

    // First check if there is an open transaction for this : 
    let mut configs = configs::get_config();

    if let Some(friend) = configs.friends.get_mut(&author_ed) {
        // generate key and store friend data
        let shared_key = plume_core::encryption::generate_shared_key(friend.private_x.clone(), author_x);
        friend.username = author_name;
        friend.shared_key = shared_key;
    }



    config::update_config(&configs);
}
