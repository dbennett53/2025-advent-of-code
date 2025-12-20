use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
struct Point(i64, i64, i64);

fn dist(p1: &Point, p2: &Point) -> i64 {
    return (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2)
}

fn parse_line(line: &str) -> Point {
    let mut out = line.split(',').map(|x| x.trim().parse::<i64>().unwrap());
    Point(out.next().unwrap(), out.next().unwrap(), out.next().unwrap())
}

fn parse_file(filename: &str) -> Vec<Point> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let out: Vec<Point> = reader.lines().map(|x| parse_line(&x.expect("Could not read line"))).collect();
    out
}

fn find_closest_pairs(pairs: &Vec<Point>, num: usize) -> Vec<(usize, usize, i64)> {
    let mut closest_pairs: Vec<(usize, usize, i64)> = vec![(0,0,i64::MAX); num + 1];

    for (i, p1) in pairs.into_iter().enumerate() {
        for (j, p2) in pairs[i+1..].into_iter().enumerate() {
            closest_pairs[num] = (i, j+i+1, dist(p1, p2));
            closest_pairs.sort_by_key(|x| x.2);
        }
    }

    closest_pairs.pop();
    closest_pairs
}

// Given a list of connections that have been made, determines the numbers and sizes of circuits
fn get_circuits(connections: &Vec<(usize, usize, i64)>) -> Vec<HashSet<usize>> {
    let mut circuits: Vec<HashSet<usize>> = vec![];

    for (c0, c1, _) in connections {
        let sets_containing: Vec<usize> = circuits.iter()
            .enumerate()
            .filter(|(_, x)| x.contains(&c0) || x.contains(&c1))
            .map(|(i, _)| i)
            .rev()
            .collect();
        if sets_containing.is_empty() {
            circuits.push(HashSet::from([*c0, *c1]));
        } else {
            for index in &sets_containing {
                circuits[*index].insert(*c0);
                circuits[*index].insert(*c1);
            }
            let first_index = sets_containing[sets_containing.len()-1];
            for index in sets_containing[..sets_containing.len()-1].iter() {
                let union: HashSet<usize> = circuits[first_index]
                    .union(&circuits[*index])
                    .map(|x| x.clone())
                    .collect();
                circuits[first_index] = union;
                circuits.remove(*index);
            }
        }
    }

    circuits.sort_by_key(|x| std::cmp::Reverse(x.len()));
    circuits
}


fn part_1(filename: &str, conn_num: usize, prod_num: i64) -> i64 {
    let boxes = parse_file(filename);
    let connections = find_closest_pairs(&boxes, conn_num);
    let circuits = get_circuits(&connections);
    let n = std::cmp::min(prod_num as usize, circuits.len());
    let product = circuits[0..n]
        .iter()
        .map(|x| x.len())
        .reduce(|prod, x| prod * x).unwrap() as i64;
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("57,618,57"), Point(57, 618, 57));
    }

    #[test]
    fn test_parse_file() {
        let vector = parse_file("inputs/example.txt");
        assert_eq!(vector.len(), 20);
        assert_eq!(vector[0], Point(162, 817, 812));
        assert_eq!(vector[5], Point(466, 668, 158));
        assert_eq!(vector[19], Point(425, 690, 689));
    }

    #[test]
    fn test_example() {
        assert_eq!(part_1("inputs/example.txt", 10, 3), 40);
    }
}

fn main() {
    let part_1_solution = part_1("inputs/input.txt", 1000, 3);
    println!("Part 1: {}", part_1_solution);
}
