use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

fn print_bash_icon() {
    print!("$ ")
}

fn command_not_found(command: &str) {
    println!("{}: not found", command);
}

fn find_command_in_path(cmd: &str, path: &str) -> Option<PathBuf> {
    for sub_path in path.split(":") {
        let p = Path::new(sub_path).join(cmd);
        if p.exists() {
            return Some(p);
        }
    }
    None
}
fn type_command(args: Option<&str>, path: &str) {
    match args {
        Some("exit") => println!("exit is a shell builtin"),
        Some("echo") => println!("echo is a shell builtin"),
        Some("type") => println!("type is a shell builtin"),
        Some("pwd") => println!("pwd is a shell builtin"),
        None => eprintln!("please specify your command"),
        Some(c) => {
            if let Some(pb) = find_command_in_path(c, path) {
                println!("{} is {}", c, pb.display().to_string());
            } else {
                command_not_found(c);
            }
        }
    }
}

fn echo_command(args: Option<&str>) {
    match args {
        Some(a) => println!("{}", a),
        None => println!(),
    }
}

fn run_ext_command(command: &str, args: Option<&str>, path: &str) {
    match find_command_in_path(command, path) {
        None => command_not_found(command),
        Some(_) => {
            let mut cmd = process::Command::new(command);
            if let Some(arg) = args {
                for a in arg.split(' ') {
                    cmd.arg(a);
                }
            }
            cmd.stdout(process::Stdio::piped());
            cmd.stderr(process::Stdio::piped());

            let spawn = cmd.spawn();
            if let Err(e) = spawn {
                eprintln!("error spawning a new process: {}", e);
                return;
            }
            let spawn = spawn.unwrap();
            let output = spawn.wait_with_output();
            match output {
                Ok(n) => {
                    if n.status.success() {
                        let out = String::from_utf8(n.stdout).unwrap_or_else(|e| {
                            format!("error converting output to utf-8 string: {}", e)
                        });
                        print!("{}", out);
                    } else {
                        let out = String::from_utf8(n.stderr).unwrap_or_else(|e| {
                            format!("error converting output to utf-8 string: {}", e)
                        });
                        eprint!("{}", out);
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }
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
        Some("pwd") => {
            let pwd = env::var("PWD")
                .unwrap_or_else(|e| format!("PWD environment variable not exist: {}", e));
            println!("{}", pwd);
        }
        Some(c) => run_ext_command(c, args, path),
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
