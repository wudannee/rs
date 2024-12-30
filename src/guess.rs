use std::{
    cmp::Ordering,
    io,
    time::{self, UNIX_EPOCH},
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use time::Duration;

    #[derive(Debug)]
    struct Animal {
        name: String,
        kind: String,
    }

    impl PartialEq for Animal {
        fn eq(&self, other: &Self) -> bool {
            self.kind == other.kind
        }
    }

    #[test]
    fn test_animals_eq() {
        let dog = Animal {
            name: "dog".to_string(),
            kind: "mammal".to_string(),
        };
        let cat = Animal {
            name: "cat".to_string(),
            kind: "mammal".to_string(),
        };

        let fish = Animal {
            name: "fish".to_string(),
            kind: "fish".to_string(),
        };

        let shark = Animal {
            name: "shark".to_string(),
            kind: "fish".to_string(),
        };

        assert_eq!(dog, cat);
        assert_ne!(dog, fish);
        assert_eq!(fish, shark);
    }

    #[test]
    fn test_random_number_between_1_and_100() {
        // dummy test
        for _ in 0..500 {
            let input = foo_random_number();
            assert!(
                input >= 1 && input <= 100,
                "invalid number {}, input should be between 1 and 100",
                input
            );
        }
    }

    #[test]
    #[ignore]
    // ignore this test, because it's too expensive
    // you can use `cargo test -- --ignored` to run it
    fn test_expensive_function() {
        for _ in 1..=3 {
            thread::sleep(Duration::from_secs(1));
        }
        assert!(true);
    }
}
