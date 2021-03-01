use std::fs::File;

fn main() {
    /*
        In RUST, there are 2 types of error
        - Recoverable (Result<T, E>)
        - Unrecoverable (Panic!) APPLICATION QUIT
     */
    /*
        let mut v = vec![45, 7, 10];
        v.push(11);
        println!("{}", v[5]); // PANIC!
        // Program stopped, remaining will not be executed!
        println!("Next after panic!");
     */

    let f = File::open("dummy.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => { panic!("Problem opening file {:?}", error); }
    };

    // Another way, with expect(); unwrappe() can also be used, but too verbose
    let file = File::open("score.txt").expect("Custom error message if so");
}
