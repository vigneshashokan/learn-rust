use std::collections::HashMap;

pub fn run() {
    let mut hm = HashMap::new();
    hm.insert(1, String::from("vignesh"));
    hm.insert(2, String::from("ashokan"));
    hm.insert(3, String::from("gov"));

    for (k, v) in &hm {
        println!("{}: {}", k, v);
    }

    match hm.get(&3) {
        Some(v) => println!("value: {}", v),
        _ => println!("no values found"),
    }

    hm.remove(&3);
    match hm.get(&3) {
        Some(v) => println!("value: {}", v),
        _ => println!("no values found"),
    }

}