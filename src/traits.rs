// trait Shape {
//     fn area(&self) -> u32;
// }

// struct Rectangle{
//     x: u32,
//     y: u32,
// }

// struct Circle {
//     radius: f64,
// }

// impl Shape for Rectangle {
//     fn area(&self) -> u32 {
//         self.x * self.y
//     }
// }

// impl Shape for Circle {
//     fn area(&self) -> u32 {
//         (3.14 * self.radius * self.radius) as u32
//     }
// }

#[derive(Clone, Debug)]
struct A(i32);

// #[derive(Eq, PartialEq)]
// struct B(f32);
pub fn run() {
    // let r = Rectangle {
    //     x: 30,
    //     y: 20,
    // };

    // let c = Circle {
    //     radius: 100.1332,
    // };

    // println!("Area of rectangle: {}", r.area());
    // println!("Area of circle: {}", c.area());

    let a = A(32);
    // let b = B(12.13);

    println!("a: {:?}", a);
    // println!("b: {:?}", b);
}