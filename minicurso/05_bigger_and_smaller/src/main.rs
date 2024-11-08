fn main() {
    let array = [1, 2, 3, 4, 5];
    let mut bigger = array[0];
    let mut smaller = array[0];

    for number in array.iter() {
        if *number < smaller {
            smaller = *number;
        }
        else if *number > bigger {
            bigger = *number;
        }
        
    }

    println!("The bigger number is: {}", bigger);
    println!("The smaller number is: {}", smaller);
}