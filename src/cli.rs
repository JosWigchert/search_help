use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("searchhelp")
        .about("CLI for managing and searching help texts of applications")
        .subcommand(
            Command::new("add")
                .about("Add a new program")
                .arg(Arg::new("name").required(true))
                .arg(Arg::new("command").required(true)),
        )
        .subcommand(
            Command::new("update")
                .about("Update a program by name or by selecting it from the list of programs")
                .arg(Arg::new("name").required(false)),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a program by name or by selecting it from the list of programs")
                .arg(Arg::new("name").required(false)),
        )
        .subcommand(
            Command::new("search")
                .about("Search help texts")
                .arg(Arg::new("query").required(true)),
        )
}
