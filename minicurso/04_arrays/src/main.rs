use std::io;

fn main() {
    let array = [1, 2, 3, 4, 5];

    for number in array.iter() {
        println!("{}", number);
    }

    let mut palavra = String::new();
    let scanner = io::stdin();

    scanner.read_line(&mut palavra).expect("Failed to read line");
    for character in palavra.chars() {
        println!("{}", character);
    }
}
