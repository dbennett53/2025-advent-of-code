
#[derive(PartialOrd, PartialEq, Eq, Clone)]
struct FreshRange {
    start: u64,
    end: u64,
}

struct FreshRanges {
    
    min: u64,
    max: u64,
    ranges: Vec<FreshRange>,
}

impl FreshRange{
    fn new(start: u64, end: u64) -> Self {
        FreshRange { start, end }
    }

    // Merges two ranges and modifies self with the merged range. If the ranges do not overlap, returns false.
    fn merge(&mut self, other: &FreshRange) -> bool{
        if (self.start >= other.start && self.start <= other.end) ||
           (self.end >= other.start && self.end <= other.end) ||
           (other.start >=self.start && other.start <= self.end) ||
           (other.end >= self.start && other.end <= self.end) { 
            self.start = self.start.min(other.start);
            self.end = self.end.max(other.end);
            true
        } else {
            false
        }
    }
}

impl Ord for FreshRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start).then(self.end.cmp(&other.end))
    }
}

impl FreshRanges {
    fn new() -> Self {
        FreshRanges {
            min: u64::MAX,
            max: 0,
            ranges: Vec::new(),
        }
    }

    fn add_range(&mut self, new_range: FreshRange) {
        self.min = self.min.min(new_range.start);
        self.max = self.max.max(new_range.end);

        let mut merged = false;
        for index in 0..self.ranges.len() {
            if self.ranges[index].merge(&new_range) {
                while index + 1 < self.ranges.len() {
                    let (left, right) = self.ranges.split_at_mut(index + 1);
                    if left[index].merge(&right[0]) {
                        self.ranges.remove(index + 1);
                    } else {
                        break;
                    }
                }
                merged = true;
                break;
            }

        } 
        
        if !merged {
            self.ranges.push(new_range);
        }

        self.ranges.sort();
    }

    fn check_fresh(&self, value: u64) -> bool {
        if value < self.min || value > self.max {
            return false;
        }
        for range in &self.ranges {
            if value >= range.start && value <= range.end {
                return true;
            }
        }
        false
    }
}


fn parse_file(filename: &str) -> (i32, i64) {
    let content = std::fs::read_to_string(filename).expect("Could not read file");
    let mut fresh_ranges = FreshRanges::new();

    // Get fresh ranges
    let mut count = 0;
    let mut getting_ranges = true;
    for line in content.lines() {
        if line.trim().is_empty() {
            getting_ranges = false;
            continue;
        }
        if getting_ranges {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() != 2 {
                panic!("Line ({}) did not match expected format", line);
            }
            let start = parts[0].parse::<u64>().expect("Could not parse start of range");
            let end = parts[1].parse::<u64>().expect("Could not parse end of range");
            fresh_ranges.add_range(FreshRange::new(start, end));
        } else {
            let value = line.parse::<u64>().expect("Could not parse value");
            if fresh_ranges.check_fresh(value) {
                count += 1;
            }
        }
    }

    // Get number of fresh values
    let mut total_fresh: i64 = 0;
    for range in &fresh_ranges.ranges {
        println!("Fresh range: {}-{}", range.start, range.end);
        total_fresh += (range.end - range.start + 1) as i64;
    }
    (count, total_fresh)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let test_cases = vec![
            (FreshRange::new(5, 10), FreshRange::new(8, 15), FreshRange::new(5, 15), true),
            (FreshRange::new(5, 10), FreshRange::new(3, 15), FreshRange::new(3, 15), true),
            (FreshRange::new(5, 10), FreshRange::new(3, 7), FreshRange::new(3, 10), true),
            (FreshRange::new(5, 20), FreshRange::new(8, 10), FreshRange::new(5, 20), true),
            (FreshRange::new(5, 20), FreshRange::new(4, 30), FreshRange::new(4, 30), true),
            (FreshRange::new(20, 25), FreshRange::new(30, 35), FreshRange::new(20, 25), false),
            (FreshRange::new(57, 100), FreshRange::new(30, 35), FreshRange::new(57, 100), false),
        ];
        for (mut range1, range2, expected_range, expected_result) in test_cases {
            let result = range1.merge(&range2);
            assert_eq!(result, expected_result);
            assert_eq!(range1.start, expected_range.start);
            assert_eq!(range1.end, expected_range.end);
        }   
    }

    #[test]
    fn test_example() {
        assert_eq!(parse_file("inputs/example.txt"), (3, 14));
    }
}

fn main() {
    let (part_1_count, part_2_count) = parse_file("inputs/input.txt");
    println!("Number of fresh values: {}", part_1_count);
    println!("Total number of fresh values: {}", part_2_count);
}
