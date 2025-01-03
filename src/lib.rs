pub mod guess;

/// doc test for substract
/// ```
/// use y::substract;
/// assert_eq!(substract(1, 1), 0);
/// assert_eq!(substract(1, 20), -19);
/// ```
/// Substracts two numbers
pub fn substract(a: i32, b: i32) -> i32 {
    a - b
}

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

    /// Counts the number of words in a file and displays the results
    pub fn count_words_in_file() {
        println!("==== Counting words in a file ====");
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

pub mod foo2 {
    use std::fmt::Display;

    /// dummy generic function which returns its input
    fn echo<T>(x: T) -> T {
        x
    }

    struct Container<T> {
        value: T,
    }

    impl<T> Container<T> {
        fn new(value: T) -> Container<T> {
            Self { value }
        }
    }

    /// type-specific implementation
    /// only applicable to Container<String>
    impl Container<String> {
        fn reverse(&mut self) {
            self.value = self.value.chars().rev().collect();
        }
    }

    trait Vehicle {
        fn drive(&self);
    }

    struct Benz {
        name: String,
    }

    struct BMW {
        name: String,
    }

    /// implements Default for Benz, setting a default name.
    impl Default for Benz {
        fn default() -> Self {
            Self {
                name: "benz-defaultname".to_string(),
            }
        }
    }

    /// implements From<&str> for Benz
    ///
    /// allows for the conversion from &str to Benz
    impl From<&str> for Benz {
        /// Constructs a `Benz` from a comma-separated string, using the second element as the name, defaulting to "default-from-benz".
        fn from(value: &str) -> Self {
            let default_name = "default-from-benz";
            let val = value.to_string();
            let names: Vec<&str> = val.split(",").collect();
            let name = names.get(1).unwrap_or(&default_name).to_string();
            Self { name }
        }
    }

    impl Vehicle for Benz {
        fn drive(&self) {
            println!("{} is driving", self.name);
        }
    }

    impl Vehicle for BMW {
        fn drive(&self) {
            println!("{} is driving", self.name);
        }
    }

    enum CarType {
        benz,
        bmw,
    }

    fn which_car(car_type: CarType) -> Box<dyn Vehicle> {
        match car_type {
            // CarType::benz => Box::new(Benz {
            //     name: "benz-1".to_string(),
            // }),
            // CarType::benz => Box::new(Benz::default()),
            CarType::benz => Box::new(Benz::from("name1,benz-name2,name3")),
            CarType::bmw => Box::new(BMW {
                name: "bmw-1".to_string(),
            }),
            _ => {
                panic!("invalid car type");
            }
        }
    }

    pub fn demo_trait() {
        let benz = which_car(CarType::benz);
        benz.drive();

        let benz = Benz::default();
        benz.drive();

        // Into trait is automatically implemented, since `Benz` implements the From<&str> trait
        let benz: Benz = "into_benz1,into_benz2".into();
        benz.drive();

        associated_type_and_type_parameter();

        print_out(3);
        print_out("xyz");
        print_out(benz);
    }

    impl Display for Benz {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "display=> benz: {}", self.name)
        }
    }

    fn print_out<T: std::fmt::Display>(x: T) {
        println!("{}", x);
    }

    /// Demonstrates the use of associated types and type parameters.
    ///
    /// Creates an instance of `Ford`, and calls both `get_price` and `get_value` on it,
    /// printing out the results.
    ///
    /// The `get_price` method is defined on the `Priced` trait, which `Ford` implements.
    /// The `get_value` method is not explicitly defined, but it is implicitly defined
    /// by the `Priced` trait's associated type, `Out`, which is set to `f64` for `Ford`.
    fn associated_type_and_type_parameter() {
        let ford = Ford { price: 1.23 };
        println!("price: {}, value: {}", ford.get_price(), ford.get_value());
    }
    struct Ford {
        price: f64,
    }

    impl Priced for Ford {
        type Out = f64;

        fn get_price(&self) -> Self::Out {
            self.price
        }
    }

    impl Valued<f64> for Ford {
        fn get_value(&self) -> f64 {
            self.price
        }
    }

    trait Valued<T> {
        fn get_value(&self) -> T;
    }
    trait Priced {
        type Out;
        fn get_price(&self) -> Self::Out;
    }

    pub fn demo_generic() {
        let a = echo("hello");
        println!("a: {}", a);

        let a = echo(1.23);
        println!("a: {}", a);

        let a = Container::new(1);
        println!("{}", a.value);

        let a = Container::new(vec![1, 2, 3]);
        println!("{:?}", a.value);

        let mut a = Container::new(String::from("edoc tsur"));
        a.reverse();
        println!("{}", a.value);
    }
}
