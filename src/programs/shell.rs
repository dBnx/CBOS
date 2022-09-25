//! Minell. A `MInimal shELL`.
//! Supports basic operations, like:
//! - None

use crate::prelude::*;
use crate::task;
use crate::task::keyboard::ScancodeStream;

/// Entrypoint.
pub async fn run(kb: &mut ScancodeStream) {
    eprintln!("\nMinell. A MInimal shELL.\nType help for help. Exit to exit ..");
    loop {
        eprint!("> ");
        // parse_input
        let command: String = task::keyboard::get_and_print_line(kb, 80).await;
        println!("");
        parse_command(&command).await;
    }
}

async fn parse_command(command: &str) {
    async {}.await;
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
