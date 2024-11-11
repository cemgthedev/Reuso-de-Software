fn main() {
    let p1:(i32, i32, i32) = (1, 2, 3);
    let p2:(i32, i32, i32) = (4, 5, 6);

    let distance:i32 = (p1.0 - p2.0) * (p1.0 - p2.0) + (p1.1 - p2.1) * (p1.1 - p2.1) + (p1.2 - p2.2) * (p1.2 - p2.2);

    let distance:f32 = distance as f32;

    println!("Distance: {}", distance.sqrt());
}
