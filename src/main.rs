use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process;

fn print_bash_icon() {
    print!("$ ")
}

fn command_not_found(command: &str) {
    println!("{}: not found", command);
}

fn type_command(args: Option<&str>, path: &str) {
    match args {
        Some("exit") => println!("exit is a shell builtin"),
        Some("echo") => println!("echo is a shell builtin"),
        Some("type") => println!("type is a shell builtin"),
        None => eprintln!("please specify your command"),
        Some(c) => {
            for sub_path in path.split(":") {
                let p = Path::new(sub_path).join(c);
                if p.exists() {
                    println!("{:?}", p);
                    return;
                }
            }
            command_not_found(c);
        }
    }
}

fn echo_command(args: Option<&str>) {
    match args {
        Some(a) => println!("{}", a),
        None => println!(""),
    }
}

fn process_command(full_command: &str, path: &str) {
    let mut split = full_command.splitn(2, ' ');
    let command = split.next();
    let args = split.next();
    match command {
        Some("exit") => process::exit(0),
        Some("echo") => echo_command(args),
        Some("type") => type_command(args, path),
        Some(c) => command_not_found(c),
        None => print_bash_icon(),
    }
}

fn main() {
    let path = env::var("PATH");
    if let Err(e) = path {
        eprintln!("cannot found PATH env variable: {}", e);
        process::exit(1);
    }
    let path = path.unwrap();
    loop {
        print_bash_icon();
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        if let Err(error) = stdin.read_line(&mut input) {
            eprintln!("\rstdin: failed to read. {}", error);
            process::exit(1);
        }
        process_command(input.trim(), &path);
    }
}
