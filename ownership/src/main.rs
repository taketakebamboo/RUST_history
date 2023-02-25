fn main() {
    let s1 = String::from("hello");

    let (s2, len) = colculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn colculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
