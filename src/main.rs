use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

const PATH_STR: &'static str = "/home/adam/devel/rust/csv_reader/abalone.data";

struct AbaloneResult {}

fn main() {
    let path = Path::new(PATH_STR);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    // read a line into buffer
    reader.read_line(&mut buffer);

    println!("{}", buffer);
}
