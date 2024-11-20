use std::{
    cmp::Ordering, io, time::{self, UNIX_EPOCH}
};

/// Generates a random number between 1 and 100.
///
/// The seed is the number of milliseconds since the Unix epoch.
fn foo_random_number() -> i32 {
    let seed = time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    (seed % 100 + 1) as i32
}

/// Reads a line of input from the user and returns it as a string.
///
/// # Panics
///
/// Panics if the user is unable to provide input.
fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input
}

pub fn do_guess() {
    let number = foo_random_number();
    println!("expected number: {}", number);

    // loop until the user guesses the correct number
    loop {
        println!("please guess a number: ");
        let guess: i32 = match get_user_input().trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // if the parsing fails, read the input again
        };

        // compare the guess with the secret number
        match guess.cmp(&number) {
            Ordering::Equal => {
                println!("you got it: {guess}");
                break; // exit the loop if the guess is correct
            }
            not_equal => {
                println!("==> {guess} is not correct");
                match not_equal {
                    Ordering::Greater => println!("too big"),
                    Ordering::Less => println!("too small"),
                    _ => unreachable!("unreachable"),
                }
            }
        }
    }
}
