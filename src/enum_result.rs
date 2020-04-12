use std::fs::File;

pub fn run() {
    let f = File::open("sample.txt");

    let _ = match f {
        Ok(file) => file,
        Err(e) => panic!("Error: {:?}", e),
    };
}