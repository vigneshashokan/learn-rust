pub fn run() {
    // let x = 5;
    // match x {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     4 => println!("four"),
    //     5 => println!("five"),
    //     _ => println!("not matched"),
    // }

    // let num = 15;
    // match num {
    //     1 => println!("one"),
    //     2 | 3 | 5 | 7 | 11 => println!("num is prime"),
    //     12..=19 => println!("num is teen"),
    //     _ => println!("not matched"),
    // }

    // let t = (0, -5);
    // match t {
    //     (x, 0) => println!("x is {}", x),
    //     (0, y) => println!("y is {}", y),
    //     _ => println!("not matched"),
    // }

    // let t = (5, -5);
    // match t {
    //     (x, y) if x == y => println!("x equal to y"),
    //     (x, y) if x + y == 0 => println!("sum is zero"),
    //     (x, _) if x % 2 == 0 => println!("x is even"),
    //     _ => println!("not matched"),
    // }

    let num = 15;
    match num {
        num @ 0..=9 => println!("single digit: {}", num),
        num @ 10..=99 => println!("double digit: {}", num),
        _ => println!("not matched"),
    }

}