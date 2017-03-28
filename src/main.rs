use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::vec::Vec;

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

/// Process the dataset
fn process(path: &Path) -> Vec<Vec<String>> {
    let mut rows = Vec::<Vec<String>>::new();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    for (line_no, line) in reader.lines().enumerate() {
        let errmsg = format!("Failed to unwrap {}:{}", path.display(), line_no);
        let row: Vec<String> = line.expect(errmsg.as_str())
            .split(',')
            .map(String::from)
            .collect();
        rows.push(row);
    }

    rows
}

fn main() {
    let path = Path::new(PATH_STR);

    let abalones = process(path);

    println!("{:#?}", &abalones[2..5]);
}
