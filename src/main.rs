use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

const PATH_STR: &'static str = "/home/adam/devel/rust/csv_reader/abalone.data";

#[derive(Debug)]
struct AbaloneResult {
    sex: char,
    length: f32,
    diameter: f32,
    height: f32,
    w_whole: f32,
    w_shucked: f32,
    w_viscera: f32,
    w_shell: f32,
    rings: i32,
}

impl AbaloneResult {
    fn new(sex: char,
           length: f32,
           diameter: f32,
           height: f32,
           w_whole: f32,
           w_shucked: f32,
           w_viscera: f32,
           w_shell: f32,
           rings: i32)
           -> AbaloneResult {
        AbaloneResult {
            sex: sex,
            length: length,
            diameter: diameter,
            height: height,
            w_whole: w_whole,
            w_shucked: w_shucked,
            w_viscera: w_viscera,
            w_shell: w_shell,
            rings: rings,
        }
    }
}

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
