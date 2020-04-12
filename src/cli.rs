use std::env;

pub fn run() {
    let args:Vec<String> = env::args().collect();

    let command = args[1].clone();

    if command == "files" {
        println!("shows files");
    }
}