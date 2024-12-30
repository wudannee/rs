use y::foo1;
use y::guess;
use y::word_counter;

fn main() {
    // foo1::greeting();
    let result = y::substract(10, 5);
    println!("result of substract(10, 5): {}", result);
    foo1::hashmap();
    word_counter::count_words_in_file();
    // guess::do_guess();
}
