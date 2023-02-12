use std::io;

fn main() {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = number.trim().parse().expect("Pleese type number!");

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
