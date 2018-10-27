use std::io;
use std::io::prelude::*;
use std::fs;
use std::process::Command;
use std::env;
use std::path::Path;

fn  change_directory(path: &str){
    let current_path = Path::new(path);
    env::set_current_dir(&current_path);
    
}

fn list_items(){
    let paths = fs::read_dir("/home/phonnz/").unwrap();

    for path in paths {
            println!("{:?}", path);
    }
}

fn parse_cmd(cmd: &String) -> Vec<&str> {
    let args = cmd.trim().split_whitespace();
     args.collect::<Vec<&str>>()
    
}

fn make_queue(command: &String) -> Vec<&str>{
    let commands = command.trim().split(";");
    commands.collect::<Vec<&str>>()
}

fn get_prompt() -> String {
    let mut cmd_to_exec = Command::new("pwd").output().unwrap();
    let mut output = String::from("phonnz@intruso:");
    output.push_str(&String::from_utf8_lossy(&cmd_to_exec.stdout).to_string());
    output
}

fn run_cmd(command: &String){
    let mut args = parse_cmd(command);
    match args[0] {
        // "ls" => {
        //     list_items();
        // },
        "exit" => println!("Close the shell with `exit()`\n"),
        "exit()" => {
                println!("Ciao!");
                std::process::exit(1);
            },
        "cd" => {
            if args.len() > 1 {
                change_directory(args[1]);
            }else{
                change_directory("/home/phonnz");
            }
        },
        _ => {
            let mut cmd_to_exec = Command::new(args[0]);
            if args.len() > 1 {
                args.reverse();
                args.pop();
                for a in args{
                    cmd_to_exec.arg(a);
                }
            }
            cmd_to_exec.status().expect("failed to execute process");
            // list_dir.current_dir("/");
            // list_dir.status().expect("failed to execute process");
            
        }
        // ,
        // _ => println!("command not found: {}", args[0])
    }
    print!("{}", get_prompt());
    
}



fn main() {
    print!("{}", get_prompt());
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if  line.is_empty() {
            println!("Uh?");
            print!("{}", get_prompt());
        }else{
            let queue = make_queue(&line);
                for cmd in queue.iter(){
                    run_cmd(&cmd.to_string());
                }

        }
        
    }
}
