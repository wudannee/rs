use std::io::{self, ErrorKind};
use std::{fs, process};

/// Reads the contents of a file at `path` into a string.
///
/// If the file is not found, the returned error is an `io::Error` with
/// `ErrorKind::NotFound` and a message that includes `path`.
///
/// note: this function is not recommended, good to use read_file2
fn read_file1(path: &str) -> Result<String, String> {
    let mut errored = false;
    let mut err = io::Error::new(ErrorKind::NotFound, "");
    let content = fs::read_to_string(path).unwrap_or_else(|e| match e.kind() {
        ErrorKind::NotFound => {
            errored = true;
            err = io::Error::new(ErrorKind::NotFound, format!("path not found: {}", path));
            "".to_string()
        }
        _ => {
            errored = true;
            err = io::Error::new(e.kind(), e);
            "".to_string()
        }
    });

    if errored {
        // Err(format!("<raw error: {err}>, File not found: {}", path))
        Err(format!("{}", err))
    } else {
        Ok(content)
    }
}

/// Reads the contents of a file at `path` into a string.
///
/// If the file is not found, the returned error is an `io::Error` with
/// `ErrorKind::NotFound` and a message that includes `path`.
fn read_file2(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path).map_err(|e| {
        if e.kind() == ErrorKind::NotFound {
            io::Error::new(e.kind(), format!("File not found: {}", path))
        } else {
            e
        }
    })
}

fn demo_read(which_one: i32) {
    // file permission for test
    // a.txt 0220
    // b.txt 0664
    let filepath = "./b.txt";
    match which_one {
        1 => {
            let content = read_file1(filepath);
            match content {
                Ok(content) => println!("{}", content),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        2 => match read_file2(filepath) {
            Ok(content) => {
                println!("{}", content);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
        _ => {
            panic!("invalid choose, you must choose 1 or 2");
        }
    }
}

fn demo_mutiple_line() {
    let msg = r#"
    line1
    susan
    18
    how are you
    what's your name
    "#;

    let mut v = Vec::new();
    for (i, mut line) in msg.lines().into_iter().enumerate() {
        line = line.trim();
        if line != "" {
            v.push(line);
            println!("{} ==> {line}", i + 1);
        }
    }

    println!("msg = {}", msg);

    println!("{v:?}");
}

fn main() {
    demo_mutiple_line();
    demo_read(2);
}
