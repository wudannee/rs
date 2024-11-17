use std::{
    collections::HashMap,
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
fn main() {
    // greeting();
    // hashmap();

    let number = foo_random_number();
    println!("expected number: {}", number);

    loop {
        println!("please guess a number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess == number {
            println!("you win");
            break;
        } else {
            println!("==> {guess} is not correct");
        }
    }
}
