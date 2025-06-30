#[test]
fn rand_test() {
    // Generate a random integer between 1 and 100
    let random_number = rand::random_range(1..100);
    println!("Generated random number: {}", random_number);

    // Verify the number is within the expected range
    assert!(random_number >= 1 && random_number <= 100);

    // Generate a random boolean
    let random_bool = rand::random_bool(0.5);
    println!("Generated random boolean: {}", random_bool);

    // Generate a random floating point number between 0.0 and 1.0
    let random_float: f64 = rand::random();
    println!("Generated random float: {}", random_float);
    assert!(random_float >= 0.0 && random_float < 1.0);
}
