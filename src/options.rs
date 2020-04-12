fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x/y)
    }
}

pub fn run() {
    let res = division(10.0, 5.0);
    match res {
        Some(d) => println!("{}", d),
        None => println!("None"),
    }
}