pub fn add_two(a: i32) -> i32{
    internal_adder(a, 2)
}

pub fn internal_adder(a: i32, b: i32) -> i32{
    a + b
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("running linux!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("not running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Checking OS again");
    if cfg!(target_os = "linux") {
        println!("definitely linux");
    } else {
        println!("not linux");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(4, internal_adder(2, 2));
    }

    #[test]
    fn it_works_again() {
        assert_ne!(5, internal_adder(2, 2));
    }

    #[test]
    #[should_panic]
    fn another() {
        assert!(true == false);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }
}