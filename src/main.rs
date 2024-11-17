use std::{
    cmp::Ordering,
    collections::{self, HashMap},
    io,
    time::{self, UNIX_EPOCH},
};

fn greeting() {
    let mut name = "rust".to_string();
    println!("i'm {}", name);

    println!("please input your name");
    name.clear();
    std::io::stdin()
        .read_line(&mut name)
        .expect("failed to read line");
    name = name.trim().to_string();
    println!("i'm {}", name);
}

fn hashmap() {
    let m = vec![
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
        ("i", 9),
        ("j", 10),
    ]
    .into_iter()
    .collect::<HashMap<&str, i32>>();

    let f: HashMap<_, _> = m.into_iter().filter(|x| x.1 % 2 == 0).collect();
    for (k, v) in f {
        println!("{}: {}", k, v);
    }
}

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

fn main() {
    // greeting();
    // hashmap();

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
