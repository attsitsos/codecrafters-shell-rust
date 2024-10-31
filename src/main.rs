#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn process_command(full_command: &str) {
    let mut split = full_command.splitn(2, ' ');
    let command = split.next();
    let args = split.next();
    match command {
        Some("exit") => process::exit(0),
        Some("echo") => if let Some(a) = args { println!("{}", a); } else { println!("");},
        Some(c) =>  println!("{}: command not found", c),
        None => println!("$ "),
    }
}

fn main() {
    // Wait for user input
    loop {

        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        if let Err(error) = stdin.read_line(&mut input) {
            eprintln!("\rstdin: failed to read. {}", error);
            process::exit(1);
        }
        process_command(input.trim());
    }
}
