use std::io;

fn main() {
    println!("Guess the number!");

    // let secret_number = rand::rng().gen_range(1..=100);

    println!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read user input");

    let guess: u32 = guess.trim().parse().expect("Failed to parse");

    println!("You guessed {guess}");
}
