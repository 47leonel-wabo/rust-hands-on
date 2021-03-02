use std::io::Write;

fn main() {
    // Read from standard Input (KeyBoard)
    let reader = std::io::stdin();
    let mut line = String::new();
    let str_len = reader.read_line(&mut line).unwrap();

    println!("Input is: {}", &line);
    println!("Length is: {}", &str_len);

    // Output
    let mut writer = std::io::stdout(); // Standard output
    let b = writer.write("abc\n".as_bytes()).unwrap();

    println!("Number of bytes: {}", &b);
}
