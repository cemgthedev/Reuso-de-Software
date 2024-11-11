use std::io;

fn bubble_sort(numbers:&mut Vec<i32>) {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() - i - 1 {
            if numbers[j] > numbers[j + 1] {
                let temp = numbers[j];
                numbers[j] = numbers[j + 1];
                numbers[j + 1] = temp;
            }
        }
    }
}

fn main() {
    let mut numbers:Vec<i32> = Vec::new();

    loop {
        let mut input = String::new();
        println!("Please input a number or press enter to finish: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim();

        if input.is_empty() {
            break;
        }

        match input.parse::<i32>() {
            Ok(number) => numbers.push(number),
            Err(_) => println!("Please type a number!"),
        }
    }

    println!("Vetor original: {:?}", numbers);

    bubble_sort(&mut numbers);

    println!("Vetor ordenado: {:?}", numbers);
}
