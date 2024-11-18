pub mod foo1 {
    use std::{collections::HashMap, io};

    /// Prompts the user to input their name and then greets them.
    ///
    /// Initially, it greets with the default name "rust", then asks the user
    /// to input their own name, and finally greets the user with the provided name.
    pub fn greeting() {
        let mut name = "rust".to_string();
        println!("i'm {}", name);

        println!("please input your name");
        name.clear();
        io::stdin()
            .read_line(&mut name)
            .expect("failed to read line");
        name = name.trim().to_string();
        println!("i'm {}", name);
    }

    /// Creates a HashMap from a vector of tuples, filters the entries where
    /// the value is an even number, and prints the filtered key-value pairs.
    pub fn hashmap() {
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
}
