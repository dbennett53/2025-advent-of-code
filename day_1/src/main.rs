use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn parse_line(line: &str) -> i32 {
    let re = Regex::new(r"(?<sign>[LR])(?<value>\d+)").unwrap();
    let Some(caps) = re.captures(line) else {
        panic!("Line ({}) did not match expected format", line);
    };
    if &caps["sign"] == "L" {
        -caps["value"].parse::<i32>().unwrap()
    } else {
        caps["value"].parse::<i32>().unwrap()
    }
}

fn parse_file(filename: &str) -> (i32, i32) {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut sum = 50;
    let mut end_zeros: i32 = 0;
    let mut all_zeros: i32 = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let old_sum = sum;
        let intermediate_sum = sum + parse_line(line.as_str());
        sum = intermediate_sum.rem_euclid(100);
        let wraps: i32 = intermediate_sum / 100 - if intermediate_sum < 0 && old_sum != 0 { 1 } else { 0 };
        all_zeros += wraps.abs() + if intermediate_sum == 0 {1} else {0};
        
        if sum == 0 {
            end_zeros += 1;
        }
    }
    (end_zeros, all_zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(parse_file("inputs/example1.txt"), (3, 6));
    }
    #[test]
    fn example_2() {
        assert_eq!(parse_file("inputs/example2.txt"), (3, 14));
    }
}

fn main() {
    let (end_zeros, all_zeros) = parse_file("inputs/input.txt");
    println!("Zeros at the end of a turn: {}", end_zeros);
    println!("All zeros encountered: {}",all_zeros);
}
