use std::io;

fn euclidean_distance(x1: f32, y1: f32, x2: f32, y2: f32) {
    let result:f32 = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();

    println!("Euclidean distance: {}", result);
}

fn main() {
    let scanner = io::stdin();
    let mut x1_input = String::new();
    let mut y1_input = String::new();
    let mut x2_input = String::new();
    let mut y2_input = String::new();
    println!("Enter first point x1: ");
    scanner
        .read_line(&mut x1_input)
        .expect("Failed to read line");

    println!("Enter first point y1: ");
    scanner
        .read_line(&mut y1_input)
        .expect("Failed to read line");

    let x1: f32 = x1_input.trim().parse().expect("Please type a number!");
    let y1: f32 = y1_input.trim().parse().expect("Please type a number!");

    println!("Enter second point x2: ");
    scanner
        .read_line(&mut x2_input)
        .expect("Failed to read line");

    println!("Enter second point y2: ");
    scanner
        .read_line(&mut y2_input)
        .expect("Failed to read line");

    let x2: f32 = x2_input.trim().parse().expect("Please type a number!");
    let y2: f32 = y2_input.trim().parse().expect("Please type a number!");
    
    euclidean_distance(x1, y1, x2, y2);
}
