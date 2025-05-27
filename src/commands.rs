use std::process;

use plume_core::encryption;

use crate::colors;

pub fn command_list() -> Vec<String> {
    ["/exit", "/add_friend", "/help", "/request_friend"]
        .iter().map(|cmd| cmd.to_string()).collect()
}


pub fn execute_command(command: &str) {
    match command.split(' ').next().unwrap() {
        "/exit" => {
            process::exit(0);
        },
        "/help" => {
            command_list().iter().for_each(|cmd|  {
                print!("{} | ", cmd)
            });
        }
        "/request_friend" => {
            // TODO: Retrieve target_ed from the parameter
            let target_ed = String::new();

            // TODO: Retrieve author_name and author_private_ed from the config file
            let author_ed = String::new();
            let author_name = String::new();
            let author_private_ed = String::new();


            let [request_packet, author_x_pub, author_x_priv] = plume_core::relay_interaction::request_friend(target_ed, author_ed, author_name, author_private_ed);
            println!("{}", colors::info("Requesting friend"))
        },
        &_ => {
            println!("Command not implemented yet")
        }
    }
}
