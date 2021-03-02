fn main() {
    let mut num = [1, 5, 7, 8, 10];
    println!("All items = {:?}", &num);

    let mut num_iter = num.iter(); // Iterate through an array
    println!("First item = {:?}", &num_iter.next());
    println!("Second item = {:?}", &num_iter.next());
    println!("Third item = {:?}", &num_iter.next());

    // Iterate through for loop
    // iter()
    for item in num_iter {  // This will continue where it was stopped!
        println!("{}", &item);
    }

    // iter_mut()
    println!("Allow array's content to be modified - iter_mut()");
    for item in num.iter_mut() {
        *item *= 2; // Double all array items
    }
    println!("New values are {:?}", &num);
}
