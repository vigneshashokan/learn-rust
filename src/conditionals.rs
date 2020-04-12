pub fn run() {
    let age = 20;
    let country = "uk";
    if age >= 21 {
        println!("Legal to drink");
    } else if age >= 18 && country != "usa" {
        println!("Not USA, so Legal to drink");
    } else {
        println!("get out");
    }
}