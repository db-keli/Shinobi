use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("My CLI Tool")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Interacts with a server using Rust and Reqwest")
        .subcommand(Command::new("health").about("Checks the server health status"))
        .subcommand(Command::new("list-users").about("Fetches the list of users from the server"))
}
