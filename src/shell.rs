//! Minell. A MInimal shELL.
//! Supports basic operations, like:
//! - None

use crate::keyboard;
use crate::prelude::*;

/// Entrypoint.
pub fn run() {
    eprintln!("\nMinell. A MInimal shELL.\nType help for help. Exit to exit ..");
    loop {
        eprint!("> ");
        // parse_input
        let command: String = keyboard::get_new_line(80);
        println!("");
        parse_command(&command);
    }
}

fn parse_command(command: &str) {
    match command {
        "help" => print_help(),
        "shutdown" | "exit" => todo!("Not implemented yet"),
        _ => println!("Command not found. Type `help` for more information."),
    }
}

fn print_help() {
    println!("Alternatives are denoted using |");
    println!("======= Command : Function =======");
    println!("           help : Prints this.");
    println!("exit | shutdown : Shuts down the pc.");
}
