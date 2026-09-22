use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess: ");


    let guess = String::new();

    io::Stdin().read_line( guess)
}
