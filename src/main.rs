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
        Some("exit") | Some("echo") | Some("type") | Some("pwd") | Some("cd") => {
            println!("{} is a shell builtin", args.unwrap())
        }
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

fn cd_command(args: Option<&str>) {
    match args {
        None => {
            eprintln!("please specify the path you need to navigate");
        }
        Some(a) => {
            if a == "~" {
                match env::var("HOME") {
                    Ok(p) => env::set_current_dir(p).unwrap(),
                    Err(e) => eprintln!("cannot determine home directory: {}", e),
                }
                return;
            }
            let p = Path::new(a);
            if !p.exists() {
                eprintln!("cd: {}: No such file or directory", a);
                return;
            }
            if !p.is_dir() {
                eprintln!("{} is not a directory", a);
                return;
            }
            env::set_current_dir(p).unwrap();
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
            let path = env::current_dir();
            match path {
                Ok(p) => println!("{}", p.display()),
                Err(e) => eprintln!("{}", e),
            }
        }
        Some("cd") => {
            cd_command(args);
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
