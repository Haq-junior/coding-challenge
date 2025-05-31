fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..=((number as f64).sqrt() as u32) {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    // 1. Call the addition function with different values and print the results
    let sum1 = add(3, 5);
    let sum2 = add(-2, 10);
    println!("Sum of 3 and 5 is: {}", sum1);
    println!("Sum of -2 and 10 is: {}", sum2);

    // 2. Calculate and print the area of rectangles with different dimensions
    let area1 = calculate_rectangle_area(4.0, 6.0);
    let area2 = calculate_rectangle_area(2.5, 3.5);
    println!("Area of rectangle with width 4.0 and height 6.0 is: {} square units", area1);
    println!("Area of rectangle with width 2.5 and height 3.5 is: {} square units", area2);

    // 3. Test your prime number checker with several numbers
    let prime_check1 = is_prime(7);
    let prime_check2 = is_prime(10);
    println!("Is 7 a prime number? {}", prime_check1);
    println!("Is 10 a prime number? {}", prime_check2);

    // 4. Convert and print some temperatures from Fahrenheit to Celsius
    let celsius1 = fahrenheit_to_celsius(32.0);
    let celsius2 = fahrenheit_to_celsius(100.0);
    println!("32°F is equivalent to {:.2}°C", celsius1);
    println!("100°F is equivalent to {:.2}°C", celsius2);
}