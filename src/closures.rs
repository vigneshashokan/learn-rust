pub fn run() {
    let f = |i| i + 1;
    println!("{}", f(3));

    let p = || println!("this is a closure");
    p();

    let mut c = 0;

    let mut inc = || {
        c += 1;
        println!("value of c: {}", c);
    };

    inc();
    inc();
    inc();
}