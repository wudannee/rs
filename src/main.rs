use y::foo1;
use y::guess;
use y::play;
use y::word_counter;

// crate refers to main.rs

fn main() {
    // guess::do_guess();
    // foo1::greeting();
    // foo1::hashmap();
    // word_counter::count_words_in_file();
    play();
}

mod apple;
use crate::apple::eating_apple;
fn play() {
    play::hello();

    // direct call
    apple::eating_apple();
    // re-export call
    apple::eating();
}
