use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(line: &str, batteries: i32) -> i64 {
    let bank_int: Vec<i64> = line.chars().map(|c| c.to_digit(10).expect("Expected a digit") as i64).collect::<Vec<i64>>();

    let mut chosen_batteries = vec![];
    let mut min_indices = vec![];
    for i in 0..batteries {
        chosen_batteries.push(bank_int[i as usize]);
        min_indices.push(i as usize);
    }

    for i in 1..line.len() {
        let min_batt = (chosen_batteries.len() as i32 - line.len() as i32 + i as i32).clamp(0, (chosen_batteries.len() - 1) as i32);
        for j in min_batt as usize..chosen_batteries.len() {
            // Ensure we do not go backwards
            if i < j {
                continue;
            }
            if bank_int[i] > chosen_batteries[j] && i > min_indices[j] {
                chosen_batteries[j] = bank_int[i];
                min_indices[j] = i;
                for k in j+1..chosen_batteries.len() {
                    chosen_batteries[k] = bank_int[i + (k - j)];
                    min_indices[k] = i + (k - j);
                }
                break;
            }
        }
    }

    let mut pow = chosen_batteries.len() as i32 - 1;
    let joltage: i64 = chosen_batteries.iter().fold(0, |acc, &val| {
        let res = acc + val * 10_i64.pow(pow as u32);
        pow -= 1;
        res
    });
    joltage
}

fn parse_file(filename: &str, batteries: i32) -> i64 {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        sum += parse_line(&line.expect("Could not read line"), batteries);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(parse_file("inputs/example.txt", 2), 357);
        assert_eq!(parse_file("inputs/example.txt", 12), 3121910778619);
    }

    #[test]
    fn line_parsing() {
        assert_eq!(parse_line("12345", 2), 45);
        assert_eq!(parse_line("98765", 2), 98);
        assert_eq!(parse_line("11111", 2), 11);
        assert_eq!(parse_line("17113779", 2), 79);
        assert_eq!(parse_line("17113779", 4), 7779);
        assert_eq!(parse_line("12345", 3), 345);
    }
}

fn main() {
    let joltage_1 = parse_file("inputs/input.txt", 2);
    let joltage_2 = parse_file("inputs/input.txt", 12);
    println!("Total joltage part 1: {}", joltage_1);
    println!("Total joltage part 2: {}", joltage_2);
}
