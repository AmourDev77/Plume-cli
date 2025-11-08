use plume_core::config::{self, FriendRequest};
use rand::Rng;

use crate::colors;

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
        "announcement" => {
            display_info!("{}", packet_split[1]);
        },
        _ => {
            display_error!("Invalid packet received : Not a valid packet type : {}", packet_split[0])
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
    let mut configs = plume_core::config::get_config();

    if let Some(friend) = configs.friends.get_mut(&author_ed) {
        // generate key and store friend data
        let shared_key = plume_core::encryption::keys::generate_shared_key(&friend.private_x, &author_x).expect("Error generating the key"); // TODO: Handle error here

        friend.username = author_name;
        friend.shared_key = shared_key;

        return
    }

    // then we create a friend request in the configs, the friend will then be able to accept the
    // friend request using an other command
    let friend_request = FriendRequest {
        friend_public_ed: author_ed,
        friend_public_x: author_x,
        username: author_name,
        profile_picture: "".to_string() // TODO: add the profile picture to the packet handle it
    };

    let mut rng = rand::rng();
    let num: u32 = rng.random_range(0..100);

    configs.friend_requests.insert(num.to_string(), friend_request);
    display_info!("You received a friend request, accept it with /friend_accept {}", num);
    
    config::update_config(&configs);
}
