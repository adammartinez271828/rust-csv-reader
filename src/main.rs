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

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut abalones = std::vec::Vec::<AbaloneResult>::new();

    for (line_no, line) in reader.lines().enumerate() {
        let errmsg_string = format!("Failed to unwrap {}:{}", path.display(), line_no);
        let errmsg = errmsg_string.as_str();
        let l = line.expect(errmsg);
        let m: Vec<&str> = l.split(',').collect();
        let n = &m[..];
        let ab = AbaloneResult::new(n[0].chars().nth(0).expect(errmsg),
                                    n[1].parse::<f32>().expect(errmsg),
                                    n[2].parse::<f32>().expect(errmsg),
                                    n[3].parse::<f32>().expect(errmsg),
                                    n[4].parse::<f32>().expect(errmsg),
                                    n[5].parse::<f32>().expect(errmsg),
                                    n[6].parse::<f32>().expect(errmsg),
                                    n[7].parse::<f32>().expect(errmsg),
                                    n[8].parse::<i32>().expect(errmsg));
        abalones.push(ab);
    }

    println!("{:?}", &abalones[2..5]);
}
