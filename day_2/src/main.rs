use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn parse_range(range: &str) -> (i64, i64) {
    let re = Regex::new(r"(?<start>\d+)-(?<end>\d+)").unwrap();
    let Some(caps) = re.captures(range) else {
        panic!("Range ({}) did not match expected format", range);
    };
    let start = caps["start"].parse::<i64>().unwrap();
    let end = caps["end"].parse::<i64>().unwrap();
    (start, end)
}

fn is_invalid(id: i64) -> bool {
    let id_str = id.to_string();
    if id_str.len().rem_euclid(2) != 0 {
        return false;
    }
    let half_len = id_str.len() / 2;
    &id_str[..half_len] == &id_str[half_len..]
}

fn get_symmetric_number(half: i64) -> i64 {
    let half_str = half.to_string();
    let symmetric_str = format!("{}{}", half_str, half_str);
    symmetric_str.parse::<i64>().unwrap()
}

fn get_invalid_ids(start: i64, end: i64) -> Vec<i64> {
    let mut invalid_ids = Vec::new();
    
    if is_invalid(start) {
        invalid_ids.push(start);
    }

    let mut start_half: i64;
    if start.to_string().len().rem_euclid(2) != 0 {
        start_half = 10_i64.pow((start.to_string().len()/2) as u32);
    } else {
        start_half = start.to_string()[..start.to_string().len()/2].parse::<i64>().unwrap();
    }

    let mut current_invalid = get_symmetric_number(start_half);

    while current_invalid < end {
        if current_invalid > start {
            invalid_ids.push(current_invalid);
        }
        start_half += 1;
        current_invalid = get_symmetric_number(start_half);
    }

    if is_invalid(end) && end != start {
        invalid_ids.push(end);
    }

    invalid_ids
}

fn parse_file(filename: &str) -> i64 {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut sum: i64 = 0;
    for line in reader.lines() {
        let line: String = line.expect("Could not read line");
        for parts in line.split(',') {
            let part = parts.trim();
            let (start, end) = parse_range(part);
            let invalid_ids = get_invalid_ids(start, end);
            sum += invalid_ids.iter().sum::<i64>();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_ids() {
        assert!(is_invalid(1212));
        assert!(!is_invalid(101));
        assert!(is_invalid(123123));
        assert!(!is_invalid(1234));
        assert!(is_invalid(9999));
        assert!(!is_invalid(123456));
    }

    #[test]
    fn example_1_ids() {
        let file = File::open("inputs/example.txt").expect("Could not open file");
        let reader = BufReader::new(file);
        let mut invalid_ids: Vec<i64> = vec![];
        for line in reader.lines() {
            let line: String = line.expect("Could not read line");
            for parts in line.split(',') {
                let part = parts.trim();
                let (start, end) = parse_range(part);
                invalid_ids.extend(get_invalid_ids(start, end));
            }
        }      

        assert_eq!(invalid_ids, vec![11,22,99,1010,1188511885,222222,446446,38593859]);  
    }

    #[test]
    fn example_1_sum() {
        assert_eq!(parse_file("inputs/example.txt"), 1227775554);
    }
}

fn main() {
    println!("Sum of invalid IDs: {}",parse_file("inputs/input.txt"));

}
