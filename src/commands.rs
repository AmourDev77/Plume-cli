use std::process;

use crate::add_friend;

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
            add_friend();
        },
        &_ => {
            println!("Command not implemented yet")
        }
    }
}
