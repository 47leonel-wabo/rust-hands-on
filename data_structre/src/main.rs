fn main() {
    let leo = Person {
        username: String::from("leo ka"),
        email: String::from("leoka@mymail.xyz"),
        age: 28,
        is_active: true,
    };

    let lea = Person {
        username: String::from("lea Ny"),
        age: 22,
        ..leo // Fill the remaining fields with leo's data corresponding
    };

    // println!("{}", leo.email);
    println!("{}", lea.username);
    println!("{}", lea.age);
    println!("{}", lea.email);
}

struct Person {
    username: String,
    email: String,
    age: usize,
    is_active: bool,
}