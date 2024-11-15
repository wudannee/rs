use std::collections::HashMap;

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
fn main() {
    // greeting();
    hashmap();
}
