#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Right(Point),
    Left(Point),
}

impl Direction {
    fn match_direction(&self) -> Key {
        match *self {
            Direction::Up(_) => Key::UpKey(String::from("Pressed w")),
            Direction::Down(_) => Key::DownKey(String::from("Pressed s")),
            Direction::Right(_) => Key::RightKey(String::from("Pressed d")),
            Direction::Left(_) => Key::LeftKey(String::from("Pressed a")),
        }
    }
}

#[derive(Debug)]
enum Key {
    UpKey(String),
    DownKey(String),
    RightKey(String),
    LeftKey(String),
}

impl Key {
    fn destruct(&self) -> &String{
        match *self {
            Key::UpKey(ref s) => s,
            Key::DownKey(ref s) => s,
            Key::RightKey(ref s) => s,
            Key::LeftKey(ref s) => s,
        }
    } 
}
pub fn run() {
    let u = Direction::Up(Point{x: 0, y: 1});
    let k = u.match_direction();
    let x = k.destruct();
    println!("{:?}", x);
}
