use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command};

fn main() {
    loop {
        // use the `༼ つ ◕_◕ ༽つ` character as the prompt
        // need to explicitly flush this to ensure it prints before "read_line"
        print!("༼ つ ◕_◕ ༽つ ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // everything after the first whitespace character
        //     is interpreted as args to the command
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "help" => println!("I refuse to help."),
            "cd" => {
                // TODO: Do we need a default for the "cd" command?

                // default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            command => {
                let mut child = Command::new(command).args(args).spawn();

                // don't accept another command until this one completes
                match child {
                    Ok(mut child) => {
                        // don't accept another command until this one completes
                        child.wait();
                    }
                    Err(e) => eprintln!("Error: {}", e), // TODO: add more descriptive error message
                };
            }
        }
    }
}
