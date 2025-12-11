use std::fs::File;
use std::io::{BufRead, BufReader};

enum LineType {
    Data(Vec<i64>),
    Operation(Vec<char>),
    Error(String),
}

fn parse_line(line: &str) -> LineType {
    let re = regex::Regex::new(r"(\d+)").unwrap();
    let data: Vec<i64> = re.find_iter(line).map(|mat| mat.as_str().parse::<i64>().unwrap()).collect();
    if !data.is_empty() {
        return LineType::Data(data);
    }

    let re_op = regex::Regex::new(r"([+\-*\/])").unwrap();
    let operations: Vec<char> = re_op.find_iter(line).map(|mat| mat.as_str().chars().next().unwrap()).collect();
    if !operations.is_empty() {
        return LineType::Operation(operations);
    }
    return LineType::Error(format!("Line did not match expected formats: {}", line));
}

fn parse_file_part1(filename: &str) -> (Vec<Vec<i64>>, Vec<char>) {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<i64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        match parse_line(line.as_str()) {
            LineType::Data(d) => {
                for (index, entry) in d.iter().enumerate() {
                    if data.len() <= index {
                        data.push(Vec::new());
                    }
                    data[index].push(*entry);
                }
            },
            LineType::Operation(ops) => {
                operations = ops;
            },
            LineType::Error(err) =>  { panic!("{}", err); }
        }
    }
    (data, operations)
}

fn parse_file_part2(filename: &str) -> i64 {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut columns: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        for (i, ch) in line.chars().enumerate() {
            if columns.len() <= i {
                columns.push(String::new());
            }
            columns[i].push(ch);
        }
    }

    let mut sum = 0;
    let mut intermediate_sum = 0;
    let mut operation = ' ';
    for col in columns {
        if col.ends_with('+') || col.ends_with('*') {
            operation = col.chars().last().unwrap();
            println!("Found operation: {}", operation);
            intermediate_sum = col[..col.len()-1].trim().parse::<i64>().expect("Could not parse column value");
            println!("Initial intermediate sum: {}", intermediate_sum);
        } else if !col.trim().is_empty() {
            println!("Found value: {}", col);
            let value = col.trim().parse::<i64>().expect("Could not parse column value");
            intermediate_sum = match operation {
                '+' => intermediate_sum + value,
                '*' => intermediate_sum * value,
                _ => panic!("Unsupported operation: {}", operation),
            };
            println!("Updated intermediate sum: {}", intermediate_sum);
        } else {
            sum += intermediate_sum;
            intermediate_sum = 0;
            operation = ' ';
            println!("End of column group, added to sum. Current sum: {}", sum);
        }
    }
    sum += intermediate_sum;
    sum
}

fn part_1(filename: &str) -> i64 {
    let (data, operations) = parse_file_part1(filename);

    let mut sum = 0;
    for (data_row, op) in data.iter().zip(operations.iter()) {
        sum += match op {
            '+' => data_row.iter().sum::<i64>(),
            '*' => data_row.iter().product::<i64>(),
            _ => panic!("Unsupported operation: {}", op),
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_data() {
        assert!(matches!(parse_line("42  100    7  "), LineType::Data(x) if x == vec![42, 100, 7]));
        assert!(matches!(parse_line("*   +  *    + "), LineType::Operation(x) if x == vec!['*', '+', '*', '+']));
        assert!(matches!(parse_line("No valid data here"), LineType::Error(_)));
    }

    #[test]
    fn test_example() {
        assert_eq!(part_1("inputs/example.txt"), 4277556);
        assert_eq!(parse_file_part2("inputs/example.txt"), 3263827);
    }
}

fn main() {
    let result = part_1("inputs/input.txt");
    let result2 = parse_file_part2("inputs/input.txt");
    println!("Sum part 1: {}", result);
    println!("Sum part 2: {}", result2);
}
