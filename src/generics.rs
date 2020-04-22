use std::fmt;
struct Square<T> {
    x: T,
}

fn p<T: fmt::Debug>(x: T) {
    println!("{:?}", x);
}

struct A<T> {
    x: T,
}

impl <T> A<T> {
    fn item(&self) -> &T {
        &self.x
    }
}

pub fn run() {
    let s1 = Square {
        x: 5,
    };
    let s2 = Square {
        x: 6.7,
    };
    let s3 = Square {
        x: "this is a square",
    };

    p(s1.x);
    p(s2.x);
    p(s3.x);
}