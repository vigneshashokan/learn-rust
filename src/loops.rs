pub fn run() {
    // let mut count = 0;
    // loop {
    //     count += 1;
    //     println!("{}", count);

    //     if count == 10 {
    //         break;
    //     }
    // }

    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("FizzBuzz");
    //     } else if count % 3 == 0 {
    //         println!("Fizz");
    //     } else if count % 5 == 0 {
    //         println!("Buzz");
    //     } else {
    //         println!("{}", count);
    //     }

    //     count += 1;
    // }

    // for i in 1..100 {
    //     if i % 15 == 0 {
    //         println!("FizzBuzz");
    //     } else if i % 3 == 0 {
    //         println!("Fizz");
    //     } else if i % 5 == 0 {
    //         println!("Buzz");
    //     } else {
    //         println!("{}", i);
    //     }
    // }

    'a: loop {
        println!("in loop a");
        break 'a;
    }
}