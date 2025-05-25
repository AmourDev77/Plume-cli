use std::process;

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
        &_ => {
            println!("Command not implemented yet")
        }
    }
}
