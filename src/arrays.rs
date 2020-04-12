use std::mem;

pub fn run() {
    let mut arr:[i32; 5] = [1,2,3,4,5];
    println!("{:?}", arr);

    println!("3rd element: {:?}", arr[2]);

    arr[2] = 10;

    println!("3rd element: {:?}", arr[2]);
    println!("Array length: {:?}", arr.len());
    println!("Array size: {:?}", mem::size_of_val(&arr));

    let slice = &arr[1..3];
    println!("Slice: {:?}", slice);
}