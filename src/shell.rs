use std::process::{Stdio,Command,Child};
use std::io::Write;
use std::env;
use std::path::Path;
use std::fs;
use std::str::SplitWhitespace;

fn join_string(cmd_split: SplitWhitespace) -> String {
    let mut sentence = String::new();
    let mut char_array = cmd_split;
    while let Some(word) = char_array.next(){
        if sentence != "".to_string(){
            sentence.push(' ')
        }
        sentence.push_str(word)
    }
    return sentence;
}

fn windows_shell(){
    //Prints what OS you're on
    println!("{}", env::consts::OS);
    println!("While on windows, there are limited commands. View documentation. Please invoke powershell or cmd to run commands on target.");
    loop{
        //Sets up exactly like the linux shell
        let present_working_directory = env::current_dir().unwrap();
        print!("{}> ",present_working_directory.display());
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;
        while let Some(command) = commands.next(){
            let mut cmd_split = command.trim().split_whitespace();
            let command = cmd_split.next().unwrap();
            let mut args = cmd_split;
            match command {
                //Creating commands to run as if they're built in
                "cd" => {
                    let args_clone = args.clone();
                    let change_dir = args.peekable().peek().map_or("/", |x: &&str| *x);
                    let root = Path::new(change_dir);
                    if let Err(_) = env::set_current_dir(&root) {
                        let dir_spaces = join_string(args_clone);
                        let changed_dir = Path::new(&dir_spaces);
                        if let Err(e) = env::set_current_dir(&changed_dir) {
                            eprintln!("{}",e)
                        }
                    }
                    previous_command = None;
                },
                "pwd" => {
                    let present_directory = env::current_dir().unwrap();
                    println!("{}", present_directory.display());
                    previous_command = None;
                },
                "ls" => {
                    //TODO: After testing that this error handling works, find a way to LS in dirs with spaces in the name
                    if let Err(e) = fs::read_dir("./"){
                        eprintln!("{:#?}", e) //This fixed previous crash, will tell user if folder is inaccessable
                    } else {
                        let files = fs::read_dir("./").unwrap();
                        for file in files{
                            println!("{}", file.unwrap().path().display());
                        }
                    }
                    previous_command = None;
                },
                "file" => {
                    //This should work, as None args next should match this instead of crashing my prog
                    if args.next() == None{ 
                        println!("Syntax: file <file_to_file>")
                    } else {
                        let file_type = join_string(args);
                        let file = Path::new(&file_type);
                        let path = fs::metadata(file).unwrap();
                        if path.is_file() == true {
                            println!("{} is a file.", file_type)
                        } else if path.is_dir() == true {
                            println!("{} is a directory.", file_type)
                        }
                    }
                },
                //Exit gracefully
                "exit" => return,
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

fn linux_shell(){
    //Prints what OS you're on
    println!("{}", env::consts::OS);
    loop {
        //prompt and flush, ready for new input
        let present_working_directory = env::current_dir().unwrap();
        print!("{}> ", present_working_directory.display());
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

fn check_os(){
    let operating_system = env::consts::OS;
    if operating_system == "windows"{
        windows_shell();
    } else {
        linux_shell();
    }
}
fn main() {
    check_os();
}
