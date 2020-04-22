pub fn run() {
    let b = Box::new(10);
    println!("b = {}", b);

    let y = 4;
    let x = &y;
    let z = Box::new(y);

    if *x == *z {
        println!("equal");
    }
}