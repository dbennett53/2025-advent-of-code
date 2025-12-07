use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::ops;


#[derive(Debug)]
struct InvalidId {
    pattern: i64,
    // pattern_size: i64,
    repeats: i64,
    max_value: i64,
    id: i64,
    expired: bool,
}

impl InvalidId {
    fn new(pattern: i64, repeats: i64, max_value: i64) -> Self {
        let pattern_size = pattern.to_string().len() as i64;
        let mut id: i64 = 0;
        for k in 0..repeats {
            id = id + pattern * 10_i64.pow(k as u32 * pattern_size as u32);
        }
        let abs_max_value = 10_i64.pow(pattern_size as u32 * repeats as u32) - 1;

        InvalidId {
            pattern,
            repeats,
            max_value: if max_value > abs_max_value { abs_max_value } else { max_value },
            id,
            expired: id > max_value,
        }
    }
}

impl ops::Add<i64> for InvalidId {
    type Output = InvalidId;

    fn add(self, rhs: i64) -> InvalidId {
        let pattern = self.pattern + rhs;

        InvalidId::new(pattern, self.repeats, self.max_value)
    }
}

impl ops::AddAssign<i64> for InvalidId {
    fn add_assign(&mut self, rhs: i64) {
        let pattern = self.pattern + rhs;
        let new_invalid_id = InvalidId::new(pattern, self.repeats, self.max_value);
        self.pattern = new_invalid_id.pattern;
        self.id = new_invalid_id.id;
        self.expired = new_invalid_id.expired;
    }
}

impl PartialEq for InvalidId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// This function decomposes a range of numbers into a set of InvalidIds that represent the "shapes"
// of all invalid ids that can exist in the given number range. E.g. for a 6 digit number, the
// valid formats can be a 1 digit pattern repeating 6 times, a 2 digit pattern repeating
// 3 times, or a 3 digit pattern repeating twice. Different patterns are added for each distinct number
// of digits in the range. The returned vector is sorted according to the value of the invalidid
fn decompose_into_patterns(start: i64, end: i64) -> Vec<InvalidId> {
    let number_of_digits_min = start.to_string().len();
    let number_of_digit_max = end.to_string().len();

    let mut out: Vec<InvalidId> = vec![];
    for digits in number_of_digits_min..=number_of_digit_max {
        for pattern_size in 1..=(digits / 2) {
            if digits % pattern_size == 0 {
                let abs_min_start = 10_i64.pow((pattern_size - 1) as u32);
                let pattern_from_start = start.to_string()[..pattern_size].parse::<i64>().unwrap();
                out.push(InvalidId::new(
                    if abs_min_start > pattern_from_start || digits != number_of_digits_min {abs_min_start} else {pattern_from_start},
                    (digits / pattern_size) as i64,
                    end,
                ));
            }
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

fn parse_range(range: &str) -> (i64, i64) {
    let re = Regex::new(r"(?<start>\d+)-(?<end>\d+)").unwrap();
    let Some(caps) = re.captures(range) else {
        panic!("Range ({}) did not match expected format", range);
    };
    let start = caps["start"].parse::<i64>().unwrap();
    let end = caps["end"].parse::<i64>().unwrap();
    (start, end)
}

fn parse_file(filename: &str) -> (i64, i64) {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut sum_total: i64 = 0;
    let mut sum_part_1: i64 = 0;
    for line in reader.lines() {
        let line: String = line.expect("Could not read line");
        for parts in line.split(',') {
            let part = parts.trim();
            let (start, end) = parse_range(part);
            let mut invalid_id_patterns = decompose_into_patterns(start, end);
            while invalid_id_patterns.len() > 0 {
                if invalid_id_patterns[0].expired || invalid_id_patterns[0].id > end {
                    invalid_id_patterns.remove(0);
                    continue;
                }

                let mut changed = false;
                for i in 1..invalid_id_patterns.len() {
                    if invalid_id_patterns[0] == invalid_id_patterns[i] {
                        if invalid_id_patterns[i].repeats == 2 {
                            sum_part_1 += invalid_id_patterns[i].id;
                        }
                        invalid_id_patterns[i] += 1;
                        changed = true;
                    } else if invalid_id_patterns[0].id < invalid_id_patterns[i].id {
                        break;
                    }
                }
                if changed {
                    invalid_id_patterns.sort_by(|a, b| a.id.cmp(&b.id));
                    continue;
                }

                if invalid_id_patterns[0].id >= start && invalid_id_patterns[0].id <= end {
                    sum_part_1 += if invalid_id_patterns[0].repeats == 2 { invalid_id_patterns[0].id } else { 0 };
                    sum_total += invalid_id_patterns[0].id;
                }
                
                invalid_id_patterns[0] += 1;
                invalid_id_patterns.sort_by(|a, b| a.id.cmp(&b.id));
            }
        }
    }
    (sum_part_1, sum_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_id_struct() {
        let mut invalid_id = InvalidId::new(12, 3, 1500);
        assert_eq!(invalid_id.id, 121212);
        assert_eq!(invalid_id.expired, true);
        invalid_id += 1;
        assert_eq!(invalid_id.id, 131313);
        invalid_id += 11;
        assert_eq!(invalid_id.id, 242424);

        let invalid_id = InvalidId::new(34, 2, 4000);
        assert_eq!(invalid_id.id, 3434);
        assert_eq!(invalid_id.expired, false);

        let mut invalid_id = InvalidId::new(99, 4, 10000000000);
        assert_eq!(invalid_id.id, 99999999);
        assert_eq!(invalid_id.expired, false);
        assert_eq!(invalid_id.max_value, 99999999);
        invalid_id += 1;
        assert_eq!(invalid_id.id, 100100100100);
        assert_eq!(invalid_id.expired, true);

        let invalid_id = InvalidId::new(7, 5, 10000000);
        assert_eq!(invalid_id.id, 77777);
        assert_eq!(invalid_id.expired, false);
        assert_eq!(invalid_id.max_value, 99999);
    }

    #[test]
    fn decompose_patterns() {
        let patterns = decompose_into_patterns(100, 1500);
        assert_eq!(patterns.len(), 3);
        assert_eq!(patterns[0].id, 111);
        assert_eq!(patterns[1].id, 1010);
        assert_eq!(patterns[2].id, 1111);

        let patterns = decompose_into_patterns(5000, 150000);
        assert_eq!(patterns.len(), 6);
        assert_eq!(patterns[0].id, 5050);
        assert_eq!(patterns[1].id, 5555);
        assert_eq!(patterns[2].id, 11111);
        assert_eq!(patterns[3].id, 100100);
        assert_eq!(patterns[4].id, 101010);
        assert_eq!(patterns[5].id, 111111);

        let patterns = decompose_into_patterns(10, 99);
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].id, 11);
    }

    #[test]
    fn example_1_sum() {
        assert_eq!(parse_file("inputs/example.txt"), (1227775554, 4174379265));
    }
}

fn main() {
    let (part_1, total) = parse_file("inputs/input.txt");
    println!("Sum of invalid IDs. Part 1: {}, Part 2: {}", part_1, total);

}
