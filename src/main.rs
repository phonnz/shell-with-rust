use std::io;
use std::io::prelude::*;
use std::fs;
use std::process::Command;
use std::env;
use std::path::Path;

fn  change_directory(path: &str){
    let destiny_path = Path::new(path);
    let _result = env::set_current_dir(&destiny_path);
    
}

fn clean_path(path: &mut String) -> String {
    path.replace("/", "")
}

fn list_items(){
    let current_path = get_current_path().replace("\n", "");
    let paths = fs::read_dir(Path::new(&current_path)).unwrap();
    //paths = paths.iter().map(|p| clean_path);
    for path in paths {
        // let l = (path.unwrap().path().display()).into_os_string().into_string();
        // println!("{:?}", l);
        println!("{:?}", path.unwrap().path().display());
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

fn get_current_path() -> String {
    let path = Command::new("pwd").output().unwrap();
    String::from_utf8_lossy(&path.stdout).to_string()

}

fn get_prompt() -> String {
    let mut output = String::from("phonnz@intruso:");
    output.push_str(&get_current_path());
    output
}

fn exec_cmd(commands: Vec<&str>){
    for cmd in commands.iter() {
        run_single_cmd(&cmd.to_string());
    }
}

fn run_single_cmd(command: &String){
    let mut args = parse_cmd(command);
    match args[0] {
        "cls" => {
            list_items();
        },
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
                args.reverse();
                for a in args{
                    cmd_to_exec.arg(a);
                }
            }
            let result = cmd_to_exec.status().expect("failed to execute process");
            println!("{:?}", result);
            // let result = cmd_to_exec.spawn().expect("failed to execute process");
            // println!("{:?}", result);
            // list_dir.current_dir("/");
            // list_dir.status().expect("failed to execute process");
            
        }
        // ,
        // _ => println!("command not found: {}", args[0])
    }
    print!("{}", get_prompt());
    
}

fn create_sequence(cmd: &str) -> Vec<&str>{
    let sequence = cmd.trim().split("|");
    sequence.collect::<Vec<&str>>()

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
            let commands_queue: Vec<_> = queue.iter().map(|q| create_sequence(q)).collect();
            //println!("{:?}", commands_queue);
            for cmd in commands_queue.iter(){
                exec_cmd(cmd.to_vec());
            }

        }
        
    }
}
