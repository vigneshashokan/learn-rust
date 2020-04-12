// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
    age: i32
}

impl Person {
    fn new(fname: &str, lname: &str, a: i32) -> Person {
        Person {
            first_name: fname.to_string(),
            last_name: lname.to_string(),
            age: a
        }
    }

    // Get fullname
    fn fullname(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_lastname(&mut self, lname: &str) {
        self.last_name = lname.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {

    // let mut c = Color(255, 0, 0);

    // println!("{} {} {}", c.0, c.1, c.2);

    // c.1 = 200;
    // println!("{} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("vignesh", "ashokan", 30);
    println!("{} {} is {} yrs old" , p.first_name, p.last_name, p.age);

    println!("Fullname: {}" , p.fullname());

    p.set_lastname("ashokan govindasamy");

    println!("Fullname: {}" , p.fullname());

    let (fname, lname) = p.to_tuple();

    println!("firstname: {}\nlastname: {}" , fname, lname);


}