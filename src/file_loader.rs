use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn load_file() -> io::Result<Vec<String>> {
    let arg_path = env::args()
        .nth(1)
        .expect("Failed to get data file, please add as an argument");

    let file = File::open(&arg_path)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}
