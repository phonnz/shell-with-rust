use std::io;
use std::io::prelude::*;

fn parse_cmd(arg){
    String::split(arg, "");
}

fn run_cmd(arg: String){
    let args = parse_cmd(arg);
    println!("{:?}", args);
}



fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        run_cmd(line.unwrap());
        // println!("{}", line.unwrap());
    }
}
