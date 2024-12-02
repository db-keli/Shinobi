use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("My CLI Tool")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Interacts with a server using Rust and Reqwest")
        .subcommand(Command::new("health").about("Checks the server health status"))
        .subcommand(Command::new("init").about("Initialize the CLI tool with a new configuration"))
        .subcommand(Command::new("create_project").about("Create a new project"))
        .subcommand(Command::new("allow").about("Add an allowed user to your project"))
        .subcommand(Command::new("qrcode").about("generate qrcode file for project"))
        .subcommand(Command::new("all_projects").about("List all projects"))
        .subcommand(
            Command::new("getkeys")
                .about("export keys as environment variables")
                .arg(
                    Arg::new("project")
                        .help("The project to get the key from")
                        .short('p')
                        .long("project")
                        .required(true),
                )
                .arg(
                    Arg::new("qrcode")
                        .help("provide path to qrcode file")
                        .short('q')
                        .long("qrcode")
                        .required(true),
                ),
        )
}
