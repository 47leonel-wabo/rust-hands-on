fn main() {
    let sally = Person { name: String::from("Saly"), genre: Genre::FEMALE };
    println!("{:?}", &sally);

    println!("is 29 even number: {:?}", is_even(&29));

    user_genre(&sally.genre);
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

fn user_genre(genre: &Genre) {
    match genre {
        Genre::FEMALE => { println!("Female"); }
        Genre::MALE => { println!("Male"); }
        _ => { println!("Are you human being?"); } // For anything else
    }
}