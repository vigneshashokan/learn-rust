use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

#[allow(dead_code)]
fn exit(x: Option<i32>) {
    match x {
        Some(0) => panic!("we got a 0"),
        Some(x) => println!("we got a {}, things are fine", x),
        None => println!("we got nothing"),
    }
}

#[allow(dead_code)]
pub fn run3() {
    exit(Some(1));
    exit(Some(10));
    exit(None);
    exit(Some(0));
}

#[allow(dead_code)]
pub fn run2() {
    let f = File::open("text.txt");

    let _ = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("text.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Could not create file: {:?}", e);
                },
            }
        },
        Err(error) => {
            panic!("Could not open file: {:?}", error);
        }
    };
}

#[allow(dead_code)]
fn read_file3() -> Result<String, io::Error> {
    let f = File::open("text.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

#[allow(dead_code)]
fn read_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("text.txt")?.read_to_string(&mut s)?;
    
    Ok(s)
}

fn read_file() -> Result<String, io::Error> {
    let mut f = File::open("text.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn run() {
    // run3();
    // run2();

    // let f = File::open("text.txt").unwrap();
    // println!("{:?}", f);
    // let f = File::open("text.txt").expect("could not open the file");
    // println!("{:?}", f);

    let s = read_file();
    let s = match s {
        Ok(s) => s,
        Err(e) => panic!("Error: {:?}", e),
    };
    println!("{:?}", s);
}