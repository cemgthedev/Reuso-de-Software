use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    // config variables
    let mut input = String::new();
    let mut guess: i32 = -1;
    let mut guesses: i32 = 0;
    let scanner = io::stdin();
    
    // generate random number
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..=100);

    // game loop
    while guess != random_number {
        println!("Please input your guess: ");
        scanner
            .read_line(&mut input)
            .expect("Failed to read line");

        guess = input.trim().parse().expect("Please type a number!");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Select a bigger number!"),
            Ordering::Greater => println!("Select a smaller number!"),
            Ordering::Equal => println!("You win!. It took you {} guesses.", guesses),
        }

        guesses += 1;
        input.clear();
    }
}
