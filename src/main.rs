use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            panic!("No arguments provided");
        }
        2 => {
            println!("Provided path by user: {}", args[1]);
        }
        _ => {
            panic!("Too many arguments provided");
        }
    }
    let path = &args[1];

    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    println!("Story content:");
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}
