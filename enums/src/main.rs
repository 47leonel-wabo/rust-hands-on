fn main() {
    let sally = Person { name: String::from("Saly"), genre: Genre::FEMALE };
    println!("{:?}", &sally);

    println!("is 28 even number: {:?}", is_even(&28));
}

#[derive(Debug)]
enum Genre {
    MALE,
    FEMALE,
}

#[derive(Debug)]
struct Person {
    name: String,
    genre: Genre,
}

fn is_even(number: &i32) -> Option<bool> {
    if number % 2 == 0 { Some(true) } else { None }
}