fn main() {
    // Function definition
    say_hello();
    add(4, 1.5);
    println!("{}", mult(3, 4));
}

// Functions with no return type
fn say_hello() {
    println!("Hello Rust Technology!");
}

fn add(a: i32, b: f32) -> (){
    println!("{} + {} = {}", a, b, (a as f32) + b);
}

// function returning an integer of 32 bits
fn mult(a: i32, b: i32) -> i32 {
    a * b // Implicit return value
}
