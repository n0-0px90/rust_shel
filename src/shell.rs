use std::process::{Stdio,Command,Child};
use std::io::Write;
use std::env;
use std::path::Path;
fn main() {
    loop {
        //prompt and flush, ready for new input
        print!("> ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        //Set up support for piping commands
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;
        while let Some(command) = commands.next() {
            //This sets and splits whitespace for user input
            let mut cmd_split = command.trim().split_whitespace();
            let command = cmd_split.next().unwrap();
            let args = cmd_split;
            //Matches keywords in command input
            match command {
                //Uses builtin CD shell function
                "cd" => {
                    let change_dir = args.peekable().peek().map_or("/", |x: &&str| *x);
                    let root = Path::new(change_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                    previous_command = None;
                },
                //Exits gracefully
                "exit" => return,
                //Matches all other inputs that aren't CD or Exit
                command => {
                    let stdin = previous_command.map_or(
                        Stdio::inherit(),
                        |output: Child| Stdio::from(output.stdout.unwrap())
                    );
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };
                    let output = Command::new(command).args(args).stdin(stdin).stdout(stdout).spawn();
                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }
        if let Some(mut final_command) = previous_command {
            let _ = final_command.wait();
        }
    }
}
