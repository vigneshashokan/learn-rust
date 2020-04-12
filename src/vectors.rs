use std::mem;

pub fn run() {
    let mut v:Vec<i32> = vec![1,2,3,4,5];

    v[2] = 10;

    println!("Vectors: {:?}", v);
    println!("Vectors single value: {:?}", v[2]);
    println!("Vectors length: {:?}", v.len());
    println!("Vectors size: {:?}", mem::size_of_val(&v));

    let slice = &v[1..4];
    println!("Vectors slice: {:?}", slice);

    v.push(6);
    v.push(7);
    println!("Vectors: {:?}", v);

    v.pop();
    println!("Vectors: {:?}", v);

    for val in v.iter() {
        println!("number: {}", val);
    }

    for val in v.iter_mut() {
        *val *= 10;
    }

    println!("numbers: {:?}", v);
}