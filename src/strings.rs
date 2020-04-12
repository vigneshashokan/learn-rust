pub fn run() {
    let s = "Hello";
    println!("{}", s);

    // let st = String::from("Hello");
    // let stng = &st[0..3];

    // println!("{}", stng);

    let st1 = String::from("Hello");
    let st2 = String::from(" World");
    let st3 = st1 + &st2;

    println!("{}", st3);

    // str.push(' ');

    // str.push_str("World!");

    // println!("{}", str);
    // println!("string length: {}", str.len());

    // println!("Capacity: {}", str.capacity());
    // println!("is empty? {}", str.is_empty());
    // println!("Contains: {}", str.contains("World"));
    // println!("Replace: {}", str.replace("World", "there"));

    // for word in str.split_whitespace() {
    //     println!("{}", word);
    // }
}
