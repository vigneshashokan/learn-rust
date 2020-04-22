fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn run() {
    let v = vec![1, 2, 3];

    for i in v.iter() {
        println!("i: {}", i);
    }

    let top = 10000;
    let mut c = 0;

    for n in 0.. {
        let x = n * n;
        if x >= top {
            break;
        } else if is_even(x) {
            c += x;
        }
    }

    println!("{}", c);
}