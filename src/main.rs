use std::io;

fn main() {
    println!("Hello, world!");

    println!("Please inpu your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Fail to read line");

    println!("You guessed: {}", guess);
}
