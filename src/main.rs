#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn print_bash_icon() {
    print!("$ ")
}

fn command_not_found(command: &str) {
    println!("{}: not found", command);
}

fn type_command(args: Option<&str>) {
    match args {
        Some("exit") => println!("exit is a shell builtin"),
        Some("echo") => println!("echo is a shell builtin"),
        Some("type") => println!("type is a shell builtin"),
        None => eprintln!("please specify your command"),
        Some(c) => command_not_found(c),
    }
}

fn echo_command(args: Option<&str>) {
    match args {
        Some(a) => println!("{}", a),
        None => println!(""),
    }
}

fn process_command(full_command: &str) {
    let mut split = full_command.splitn(2, ' ');
    let command = split.next();
    let args = split.next();
    match command {
        Some("exit") => process::exit(0),
        Some("echo") => echo_command(args),
        Some("type") => type_command(args),
        Some(c) => command_not_found(c),
        None => print_bash_icon(),
    }
}

fn main() {
    // Wait for user input
    loop {
        print_bash_icon();
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
