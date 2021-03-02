use std::fs::OpenOptions;
use std::io::{Read, Write};

fn main() {
    /*
        // File creation
        let mut file = std::fs::File::create("my_file.txt").expect("Unable to create file");
        // Write to file
        println!("Start writing...");

        file
            .write_all("First line.".as_bytes())
            .expect("Unable to write to file");

        file
            .write_all("\nAnother text line".as_bytes())
            .expect("Unable to write to file");

        println!("End writing...");
    */
    // Reading from file
    // Opening file
    let mut f = std::fs::File::open("my_file.txt").unwrap();
    let mut str = String::new();
    let val = f.read_to_string(&mut str).unwrap();
    println!("File content: \n{:?}", &str);
    println!("Number of bytes: {:?}", &val);
    /*
        // Another way
        let my_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("my_file.txt")
            .expect("Unable to open this file"); // For Read and Write
        */
    let mut mf = OpenOptions::new()
        .append(true) // Here, we enable append option
        .open("my_file.txt")
        .expect("File IO error"); // Appending
    mf.write_all("\n\nAppended line".as_bytes()).unwrap();
}
