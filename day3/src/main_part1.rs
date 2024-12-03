use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total: i64 = 0;
    for line in reader.lines() {
        let line = line?; // Handle potential I/O errors.
        let caps = re.captures_iter(&line);
        for cap in caps {
                let first_number = &cap[1];
                let second_number = &cap[2];
                let a: i32 = first_number.parse().unwrap();
                let b: i32 = second_number.parse().unwrap();
                let num = a * b;
                println!("a|{} x b|{} = {}", a, b, num);
                total += num as i64;
        }
    }
    println!("total of muls: {}", total);
    Ok(())
}
