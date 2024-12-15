pub fn eating_apple() {
    println!("{}", "eating apple");
}

/// re-export eating_apple
pub use eating_apple as eating;
