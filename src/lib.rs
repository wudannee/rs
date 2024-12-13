pub mod guess;

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

        // filter the entries with even values and collect them into a HashMap
        let even_map: HashMap<_, _> = m.into_iter().filter(|x| x.1 % 2 == 0).collect();
        for (k, v) in even_map.iter() {
            println!("{}: {}", k, v);
        }

        // sum the values in the even map
        let sum: i32 = even_map.iter().fold(0, |acc, x| acc + x.1);
        println!("sum of values in even map: {}", sum);

        // iterate over the first two key-value pairs
        for e in even_map.iter().take(2) {
            println!("{}: {}", e.0, e.1);
        }
    }
}

pub mod word_counter {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::prelude::BufRead;
    use std::io::BufReader;
    use std::{env, process};

    #[derive(Debug)]
    struct WordCounter(HashMap<String, u64>);

    impl WordCounter {
        fn new() -> WordCounter {
            WordCounter(HashMap::new())
        }

        fn increment(&mut self, word: &str) {
            let key = word.to_string();
            let count = self.0.entry(key).or_insert(0);
            *count += 1;
        }

        fn display(&self) {
            let m = &self.0;

            // Convert the HashMap to a Vec so we can sort it
            let mut v: Vec<_> = m.iter().collect();
            // Sort the Vec by the value in descending order, and then by the key in ascending order
            v.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
            for (key, value) in v.into_iter() {
                println!("{}: {}", key, value);
            }
        }
    }

    pub fn count_words_in_file() {
        let arguments: Vec<String> = env::args().collect();
        let filename = arguments
            .get(1)
            .or_else(|| {
                println!("Please provide a filename to count words in");
                process::exit(1);
            })
            .unwrap();
        println!("Processing file: {}", filename);
        let file = File::open(filename).unwrap_or_else(|err| {
            println!("Could not open file <{}>: {}", filename, err);
            process::exit(1);
        });
        let reader = BufReader::new(file);
        let mut word_counter = WordCounter::new();
        for line in reader.lines() {
            let line = line.expect("Could not read line");
            let words = line.split(" ");
            for word in words {
                if word == "" {
                    continue;
                } else {
                    word_counter.increment(word);
                }
            }
        }
        word_counter.display();
    }
}
