mod loop_mod {
    /// Demonstrates a loop with an inner loop and a named outer loop (`'outloop`)
    /// that can be broken out of with `break 'outloop;`.
    pub fn loop_break() {
        let mut x = 20;
        'outloop: loop {
            if x <= 0 {
                break;
            }
            println!("{} more runs to go", x);
            x -= 1;
            loop {
                if x == 10 {
                    println!("x = 10, quit loop!");
                    break 'outloop;
                } else {
                    break;
                }
            }
        }
    }

    /// Iterates over the range from 1 to 5 in reverse order, stepping by 2.
    ///
    /// Prints each element of the resulting sequence.
    pub fn loop_range() {
        let r = 1..=5;
        for i in r.rev().step_by(2) {
            println!("{}", i);
        }
    }
}

mod unit_struct_mod {
    use std::any::{Any, TypeId};

    struct ReadMode;
    struct WriteMode;

    struct File<Mode> {
        mode: Mode,
    }

    impl<Mode> File<Mode> {
        fn mode(&self) -> &Mode {
            &self.mode
        }
    }
    fn open_file<Mode>(which: Mode) -> File<Mode> {
        File { mode: which }
    }

    /// Opens a file in write mode and checks its mode type.
    ///
    /// Uses the `TypeId` trait to compare the type of the file's mode with the types of `ReadMode` and `WriteMode`.
    /// Prints the mode type, or "UnknownMode" if it is neither `ReadMode` nor `WriteMode`.
    pub fn demo_read_write_mode() {
        let file = open_file(WriteMode);

        let m = file.mode();

        if m.type_id() == TypeId::of::<ReadMode>() {
            println!("ReadMode");
        } else if m.type_id() == TypeId::of::<WriteMode>() {
            println!("WriteMode");
        } else {
            println!("UnknownMode");
        }
    }
}

fn main() {
    loop_mod::loop_break();
    loop_mod::loop_range();

    unit_struct_mod::demo_read_write_mode();
}
