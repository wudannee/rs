fn main() {
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
