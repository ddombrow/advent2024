use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn is_decreasing(a: i8, b: i8) -> bool {
    b < a
}

pub fn is_increasing(a: i8, b:i8) -> bool {
    b > a
}

pub fn is_gradual_change(a: i8, b: i8) -> bool {
    let delta = (b-a).abs();
    delta >=1 && delta <= 3
}

pub fn is_report_safe(report: String) -> bool {
    let readings = report.split(" ");
    let mut all_increasing = true;
    let mut all_decreasing = true;
    let mut all_gradual = true;
    let mut last_reading = -1i8;
    for r in readings {
        let first_reading = last_reading == -1i8;
        let r_num: u8 = r.parse().unwrap();
        if first_reading {
            last_reading = r_num as i8;
            continue;
        }
        
        if !is_gradual_change(last_reading, r_num as i8) {
            //println!("failed gradual: {}|{}={}",last_reading, r_num, (r_num as i8 - last_reading).abs());
            all_gradual = false;
            break;
        }
        if !is_increasing(last_reading, r_num as i8) {
            //println!("failed increasing: {}|{}={}",last_reading, r_num, (r_num as i8 - last_reading));
            all_increasing = false;
        }
        if !is_decreasing(last_reading, r_num as i8) {
            //println!("failed decreasing: {}|{}={}",last_reading, r_num, (r_num as i8 - last_reading));
            all_decreasing = false;
        }
        last_reading = r_num as i8;
        //print!("{}:", r_num)
    }
    let safe = all_gradual && (all_increasing || all_decreasing);
    //println!("|i:{}|d:{}|g:{}|safe:{}", all_increasing, all_decreasing, all_gradual, safe);
    if safe {
        println!("safe: {report}");
    }
    else {
        println!("unsafe: {report}");
    }
    safe
}

fn main() -> io::Result<()> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut num_safe_reports = 0;
    for line in reader.lines() {
        let line = line?; // Handle potential I/O errors.
        let safe = is_report_safe(line);
        if safe {
            num_safe_reports+=1;
        }
    } 
    println!("safe reports: {}", num_safe_reports);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety() {
        let safe = is_report_safe(String::from("41 38 40 38 39"));
        assert!(!safe);
    }
}
