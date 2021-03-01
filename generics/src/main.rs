fn main() {
    let data: MyData<i32> = MyData { value: 17 };
    println!("{}", data.value);
    let msg: MyData<String> = MyData { value: "Message for you!".to_string() };
    println!("{}", msg.value);

    println!("{}", max_generic(7, 5));

    println!("--------------- Traits  ----------------------");
    let dog = Animal { name: "Rantamplan".to_string(), age: 8 };
    dog.make_noise();
    dog.walk();
    dog.noise_level(3);
}

struct MyData<E> {
    value: E
}

fn max_generic<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        b
    } else {
        a
    }
}

// Struct definition
struct Animal {
    name: String,
    age: u32,
}

// Trait definition
trait Behavior {
    fn make_noise(&self);
    fn walk(&self);
    fn noise_level(&self, intensity: u32) {
        println!("Noise level is: {}", &intensity);
    }
}

// Implementing trait for specific struct (Animal)
impl Behavior for Animal {
    fn make_noise(&self) {
        println!("I'm {}, and I make noises", self.name);
    }

    fn walk(&self) {
        println!("{} is walking...", self.name);
    }
}