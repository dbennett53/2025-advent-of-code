#![recursion_limit = "1024"]
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_file(filename: &str) -> i32 {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut splits = 0;
    let mut last_line: Vec<char> = vec![];
    for line in reader.lines() {
        let mut this_line: Vec<char> = line.expect("Could not read line").chars().collect();
        // println!("Parsing {}, last line {}", this_line.iter().collect(), last_line.iter().collect());
        if last_line.is_empty() {
            last_line = this_line;
            continue;
        }

        for (i, character) in last_line.iter().enumerate() {
            if *character == 'S' {
                this_line[i] = '|';
            } else if *character == '|' {
                if this_line[i] == '^' {
                    splits += 1;
                    if i > 0 {
                        this_line[i-1] = '|';
                    }
                    if i < this_line.len() - 1 {
                        this_line[i+1] = '|';
                    }
                } else {
                    this_line[i] = '|';
                }
            }
        }
        
        last_line = this_line;
    }


    splits
}

// Use dynamic programming to speed this up
fn split_timeline(row: usize, col: usize, grid: &Vec<Vec<char>>, mut cache: &mut Vec<Vec<i64>>) -> i64 {
    for r in row..grid.len() {
        if grid[r][col] == '^' {
            if cache[r][col] == -1 {
                let new_value = split_timeline(r + 1, col + 1, grid, &mut cache) + split_timeline(r + 1, col - 1, grid, &mut cache);
                cache[r][col] = new_value;
            }
            return cache[r][col];
        }
    }
    return 1
}

fn parse_file_part_2(filename: &str) -> i64 {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        grid.push(line.expect("Could not read line").chars().collect());
    }

    let mut timelines: i64 = 0;
    let mut cache: Vec<Vec<i64>> = vec![vec![-1; grid[0].len()]; grid.len()];
    match grid[0].iter().position(|&c| c == 'S') {        
        Some(index) => timelines = split_timeline(0, index, &grid, &mut cache),
        None => println!("Could not find starting point"),
    }

    timelines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(parse_file("inputs/example.txt"), 21);
    }
    #[test]
    fn test_example2() {
        assert_eq!(parse_file_part_2("inputs/example.txt"), 40);
    }
}

fn main() {
    let splits = parse_file("inputs/input.txt");
    let timelines = parse_file_part_2("inputs/input.txt");
    println!("Part 1: {}", splits);
    println!("Part 2: {}", timelines);
}
