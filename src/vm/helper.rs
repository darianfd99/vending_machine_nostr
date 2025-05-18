use std::io;

pub fn read_number(text: &str) -> u64 {
    println!("{}", text);
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read the line");
    num.trim().parse().expect("failed to read number")
}

pub fn read_string(text: &str) -> String {
    println!("{}", text);
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read the line");
    name.trim().to_string()
}
