use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn is_decreasing(a: i8, b: i8) -> bool {
    b < a
}

pub fn is_increasing(a: i8, b: i8) -> bool {
    b > a
}

pub fn is_gradual_change(a: i8, b: i8) -> bool {
    let delta = (b - a).abs();
    delta >= 1 && delta <= 3
}

pub fn report_to_readings(report: String) -> Vec<i8> {
    let reading_tokens = report.split(" ");
    let mut readings = vec![];
    for r in reading_tokens {
        let reading: i8 = r.parse().unwrap();
        readings.push(reading);
    }
    readings
}

pub fn more_increasing(readings: Vec<i8>) -> bool {
    let mut last_reading = -1i8;
    let mut readings_increasing_count = 0;
    let mut readings_decreasing_count = 0;
    for r in &readings {
        let r = *r;
        let first_reading = last_reading == -1i8;
        if first_reading {
            last_reading = r;
            continue;
        }
        if is_increasing(last_reading, r) {
            readings_increasing_count += 1;
        } else {
            readings_decreasing_count += 1;
        }
        last_reading = r;
    }
    readings_increasing_count > readings_decreasing_count
}

pub fn is_report_safe(readings: Vec<i8>, mulligan: bool) -> bool {
    let mut is_series_decreasing = false;
    let mut is_series_increasing = false;
    let mut last_reading = -1i8;
    let mut bad_indexes: HashSet<i8> = HashSet::new();
    let mut index = 0i8;

    for r in &readings {
        let first_reading = last_reading == -1i8;
        let r = *r;
        if first_reading {
            last_reading = r;
            index += 1;
            continue;
        }
        if !is_series_increasing && !is_series_decreasing {
            let more_readings_increasing = more_increasing(readings.clone());
            if more_readings_increasing {
                is_series_increasing = true;
            } else {
                is_series_decreasing = true;
            }
        }

        if !is_gradual_change(last_reading, r) {
            //println!("failed gradual: {}|{}={}",last_reading, r, (r - last_reading).abs());
            bad_indexes.insert(index);
        }
        if is_series_increasing && !is_increasing(last_reading, r) {
            //println!("failed increasing: {}|{}={}",last_reading, r, (r - last_reading));
            bad_indexes.insert(index);
        }
        if is_series_decreasing && !is_decreasing(last_reading, r) {
            //println!("failed decreasing: {}|{}={}",last_reading, r, (r - last_reading));
            bad_indexes.insert(index);
        }
        index += 1;
        last_reading = r;
    }

    if bad_indexes.len() == 0 {
        return true;
    } else {
        let mut mulligan_safe = false;
        if mulligan {
            return false;
        }
        let size = readings.len();
        for i in 0..size {
            let mut mulligan_set = readings.clone();
            mulligan_set.remove(i);
            let safe_with_mulligan = is_report_safe(mulligan_set, true);
            if safe_with_mulligan {
                mulligan_safe = true;
            }
        }

        if mulligan_safe {
            return true;
        }
    }

    false
}

fn main() -> io::Result<()> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut num_safe_reports = 0;
    for line in reader.lines() {
        let line = line?; // Handle potential I/O errors.
        //println!("===========================\nreport: {}", line);
        let readings = report_to_readings(line);
        let safe = is_report_safe(readings, false);
        if safe {
            num_safe_reports += 1;
            //println!("| safe!");
        } else {
            //println!("| unsafe!");
        }
    }
    println!("safe reports: {}", num_safe_reports);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_ex1() {
        let report = String::from("7 6 4 2 1");
        println!("report: {}", report);
        let safe = is_report_safe(report_to_readings(report), false);
        assert!(safe);
    }

    #[test]
    fn test_safety_ex2() {
        let report = String::from("1 2 7 8 9");
        println!("report: {}", report);
        let safe = is_report_safe(report_to_readings(report), false);
        assert!(!safe);
    }

    #[test]
    fn test_safety_ex3() {
        let report = String::from("9 7 6 2 1");
        println!("report: {}", report);
        let safe = is_report_safe(report_to_readings(report), false);
        assert!(!safe);
    }

    #[test]
    fn test_safety_ex4() {
        let report = String::from("1 3 2 4 5");
        println!("report: {}", report);
        let safe = is_report_safe(report_to_readings(report), false);
        assert!(safe);
    }

    #[test]
    fn test_safety_ex5() {
        let report = String::from("8 6 4 4 1");
        println!("report: {}", report);
        let safe = is_report_safe(report_to_readings(report), false);
        assert!(safe);
    }

    #[test]
    fn test_safety_ex6() {
        let report = String::from("1 3 6 7 9");
        println!("report: {}", report);
        let safe = is_report_safe(report_to_readings(report), false);
        assert!(safe);
    }
}
