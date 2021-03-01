fn main() {
    // Control flow
    let a = 17;

    if a < 100 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    let player_life = 7;
    let life = if player_life >= 1 { "alive" } else { "died" };
    println!("{}", life);
    println!("---------------------------- BASIC LOOP ------------------");
    // Loop
    loop {
        println!("Infinite loop...");
        break;
    }
    println!("---------------------------- WHILE LOOP ------------------");
    // While
    let mut index = 5;
    while index >= 1 {
        println!("index = {}", index);
        index -= 1;
    }
    println!("---------------------------- FOR LOOP ------------------");
    // For
    let week = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    for day in &week { // Using "&"
        println!("{}", day);
    }
    println!();
    for n in [10, 70, 20, 50].iter() { // Using "iter()"
        println!("{}", n);
    }
}
