fn fahrenheit_to_celsius(f: f64) -> f64 {
    // same math, different order
    (5.0 / 9.0) * (f - 32.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    // same math written a bit differently
    (c * 9.0 / 5.0) + 32.0
}

fn is_even(n: i32) -> bool {
    // simple even check
    n % 2 == 0
}

fn check_guess(guess: i32, secret: i32) -> i32 {
    // return 0 (correct), 1 (too high), -1 (too low)
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    // Assignment 1 - Temperature Conversion 
    println!(" Assignment 1");

    let f0 = 32.0;
    println!("{}°F = {}°C", f0, fahrenheit_to_celsius(f0));

    // next 5 integer Fahrenheit values
    for offset in 1..=5 {
        let f = f0 as i32 + offset;
        println!("{}°F = {}°C", f, fahrenheit_to_celsius(f as f64));
    }

    // Assignment 2 - Number Analyzer
    println!("\n Assignment 2: Number Analyzer ");
    let nums = [3, 5, 15, 2, 8, 9, 10, 7, 6, 11];

    // fizzbuzz + even/odd
    for &n in nums.iter() {
        if n % 15 == 0 {
            println!("{}: FizzBuzz", n);
        } else if n % 3 == 0 {
            println!("{}: Fizz", n);
        } else if n % 5 == 0 {
            println!("{}: Buzz", n);
        } else if is_even(n) {
            println!("{}: even", n);
        } else {
            println!("{}: odd", n);
        }
    }

    // sum using a while loop
    let mut sum = 0;
    let mut i = 0;
    while i < nums.len() {
        sum += nums[i];
        i += 1;
    }
    println!("Sum = {}", sum);

    // max (simpler than a manual loop/break)
    let mut max_val = nums[0];
    for &n in &nums[1..] {
        if n > max_val {
            max_val = n;
        }
    }
    println!("Max = {}", max_val);

    // Assignment 3 - Guessing Game
    println!("\n Assignment 3: Guessing Game ");

    let secret = 42;
    let guesses = [10, 60, 45, 40, 42];

    let mut attempts = 0;
    for &g in guesses.iter() {
        attempts += 1;
        let r = check_guess(g, secret);

        if r == 0 {
            println!("{} is correct!", g);
            break;
        } else if r == 1 {
            println!("{} is too high", g);
        } else {
            println!("{} is too low", g);
        }
    }

    println!("It took {} guesses.", attempts);
}
