use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!"); // 数字を当ててください

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess."); //予想を入力してください

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); //行の読み込みに失敗しました

    println!("You guessed: {}", guess); //次のように予想しました
}
