pub fn run() {
    greeting("Hello", "Vignesh");
    let s = add(1, 2);
    println!("Sum: {}", s);

    let sum = |n1: i32, n2: i32| n1 + n2;
    println!("Closure Sum: {}", sum(5, 4));

}

fn greeting(greet: &str, name: &str) {
    println!("{} {}!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}