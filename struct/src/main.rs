fn main() {
    let leo = Person {
        username: String::from("leo ka"),
        email: String::from("leoka@mymail.xyz"),
        age: 28,
        is_active: true,
    };
    // println!("{}", leo.username);
    leo.introduce_yourself();

    let lea = Person {
        username: String::from("lea Nz"),
        age: 22,
        ..leo // Fill the remaining fields with leo's data corresponding
    };
    lea.introduce_yourself();

    // println!("{}", lea.username);
    // println!("{}", lea.age);
    // println!("{}", lea.email);

    let red = ColorRgb(255, 0, 0);
    println!("{}", red.0);
}

struct Person {
    username: String,
    email: String,
    age: usize,
    is_active: bool,
}

impl Person {
    fn introduce_yourself(&self) {
        println!("username: {}, age: {}, email: {}, isActice: {}", self.username, self.age, self.email, self.is_active);
    }
}

struct ColorRgb(i32, i32, i32);