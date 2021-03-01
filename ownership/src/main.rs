fn main() {
    {
        let _a = 8; // Here, the value '8' belong to 'a' (Store in the stack)
        println!("{}", _a);
    } // a' scope closed
    /*
    println!("Hello, {}", a); // 'a' is no longer available: ERROR
     */

    println!("------------ Moving Owner  -------------");
    let mut s1 = String::from("Hello"); // Here, 's1' holds the ownership of 'hello'
    s1.push_str(", Rust!");
    println!("{}", s1);
    let s2 = s1; // MOVE, 's1' is no longer the owner, It's 's2'
    println!("{}", s2);
    /*
        If we try this:
        println!("{}", s1);
        it will result an error!!!
     */
    println!("------------ Capture Ownership  -------------");
    let s = String::from("Owner captured!");
    capture_ownership(s);
    //println!("{}", s); // ERROR AS OWNER WAS MOVED

    println!("------------ Cloning Owner  -------------");
    let mut a = String::from("a value"); // Store in a heap
    a.push_str(", more text");
    let mut b = a.clone();
    b.push_str(", added by b");
    println!("{}", a);
    println!("{}", b);

    println!("------------ Copying Owner  -------------");

}

fn capture_ownership(str: String){
    println!("{}", str);
}