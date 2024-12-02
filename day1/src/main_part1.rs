use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct OfficePairs {
    pub l: u32,
    pub r: u32,
    pub distance: u32
}


fn main() -> io::Result<()> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut lefts: Vec<u32> = vec![];
    let mut rights: Vec<u32> = vec![];
    // Iterate over each line in the file.
    for line in reader.lines() {
        let line = line?; // Handle potential I/O errors.
        //println!("line {}", line);
        let iter = line.split("   ");

        let mut is_left = true;
        for i in iter {
            match i.parse::<u32>() {
                Ok(num) => {
                    if is_left {
                        lefts.push(num);
                    } else {
                        rights.push(num);
                    }
                    is_left = !is_left;
                },
                Err(e) => println!("Failed to convert: {}", e),
            }
        }
    }

    lefts.sort();
    rights.sort();
    rights.reverse();

    let mut d = 0;
    for l in lefts {
        let r = rights.pop().unwrap();
        let diff = (l as i32 - r as i32).abs() as u32;
        println!("l: {} r: {} dist: {}", l, r, diff);
        d += diff;
    }

    println!("{}", d);

    Ok(())
}
