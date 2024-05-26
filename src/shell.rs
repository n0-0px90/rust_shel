use std::process::Command;
use std::io::{Read,Write};
use std::env;
use std::path::Path;
fn main() {
    loop {
        //prompt and flush, ready for new input
        print!("> ");
        std::io::stdout().flush();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        //This sets and splits whitespace for user input
        let mut cmd_split = input.trim().split_whitespace();
        let command = cmd_split.next().unwrap();
        let args = cmd_split;
        //Matches keywords in command input
        match command {
            //Uses builtin CD command
            "cd" => {
                let change_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(change_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            //Exits sucessfully
            "exit" => return,
            command => {
                let cmd = Command::new(command).args(args).spawn();
                //Runs command, prints error gracefully instead of discombobulating
                match cmd {
                    Ok(mut cmd) => { cmd.wait(); },
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}
