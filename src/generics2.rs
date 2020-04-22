use std::ops::Mul;

trait Shape<T> {
    fn area(&self) -> T;
}

struct Rectangle<T: Mul> {
    x: T,
    y: T,
}

impl<T: Copy> Shape<T> for Rectangle<T>
where
    T: Mul<Output = T>,
{
    fn area(&self) -> T {
        self.x * self.y
    }
}

pub fn run() {
    let r = Rectangle { x: 10, y: 20 };
    let r2 = Rectangle { x: 10.10, y: 20.31 };

    println!("{} {}", r.area(), r2.area());
}
