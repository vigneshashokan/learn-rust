pub fn run() {
    let mut s = Some(0);

    if let Some(i) = s {
        println!("value: {}", i);
    }

    while let Some(i) = s {
        if i < 19 {
            println!("value: {}", i);
            s = Some(i + 2);
        } else {
            s = None;
        }
    }
}