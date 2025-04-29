use std::process;

pub fn command_list() -> Vec<String> {
    ["/exit", "/add_friend", "/help"]
        .iter().map(|cmd| cmd.to_string()).collect()
}


pub fn execute_command(command: &str) {
    match command {
        "/exit" => {
            process::exit(0);
        },
        "/help" => {
            command_list().iter().for_each(|cmd|  {
                print!("{} | ", cmd)
            });
        }
        "/add_friend" => {
            // First generate and display the keys :
            let (_, public_key ) = plume_core::encryption::generate_keys();

            println!("Transmit this private key to add a friend : {}", public_key)

            // next input will be dedicated to input x25519 key
        },
        &_ => {
            println!("Command not implemented yet")
        }
    }
}
