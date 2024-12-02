use std::fs::File;
use std::io::{self, BufRead, BufReader};


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

    /*lefts.sort();
    rights.sort();
    rights.reverse();*/

    let mut s = 0;
    for l in &lefts {
        let mut samesy = 0;
        for r in &rights {
            if r == l {
                samesy += 1;
            }
        }
        println!("l:{} s:{} ss:{}", l, samesy, l*samesy);
        s+= l * samesy;
    }

    println!("{}", s);

    Ok(())
}
