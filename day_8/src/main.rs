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

fn find_closest_pairs(pairs: &Vec<Point>, num: i64) -> Vec<(usize, usize, i64)> {
    // Return all pairs if number < 0
    if num < 0 {
        let mut closest_pairs: Vec<(usize, usize, i64)> = Vec::new();
        for (i, p1) in pairs.iter().enumerate() {
            for (j, p2) in pairs[i+1..].iter().enumerate() {
                closest_pairs.push((i, j+i+1, dist(p1, p2)));
            }
        }
        closest_pairs.sort_by_key(|x| x.2);
        return closest_pairs;
    } else {
        let mut closest_pairs: Vec<(usize, usize, i64)> = vec![(0,0,i64::MAX); num as usize + 1];

        for (i, p1) in pairs.into_iter().enumerate() {
            for (j, p2) in pairs[i+1..].into_iter().enumerate() {
                closest_pairs[num as usize] = (i, j+i+1, dist(p1, p2));
                closest_pairs.sort_by_key(|x| x.2);
            }
        }

        closest_pairs.pop();
        closest_pairs
    }
}

fn connect_circuit(circuits: &mut Vec<HashSet<usize>>, p1: usize, p2: usize) {
    let sets_containing: Vec<usize> = circuits.iter()
        .enumerate()
        .filter(|(_, x)| x.contains(&p1) || x.contains(&p2))
        .map(|(i, _)| i)
        .rev()
        .collect();
    if sets_containing.is_empty() {
        circuits.push(HashSet::from([p1, p2]));
    } else {
        for index in &sets_containing {
            circuits[*index].insert(p1);
            circuits[*index].insert(p2);
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

// Given a list of connections that have been made, determines the numbers and sizes of circuits
fn connect_circuit_list(connections: &Vec<(usize, usize, i64)>) -> Vec<HashSet<usize>> {
    let mut circuits: Vec<HashSet<usize>> = vec![];

    for (c0, c1, _) in connections {
        connect_circuit(&mut circuits, *c0, *c1);
    }

    circuits.sort_by_key(|x| std::cmp::Reverse(x.len()));
    circuits
}


// Connects circuits until there are no unconnected boxes. Returns the indices of the last connection made
fn connect_all_circuits(connections: &Vec<(usize, usize, i64)>, num_boxes: usize) -> (usize, usize) {
    let mut circuits: Vec<HashSet<usize>> = (0..num_boxes)
        .into_iter()
        .map(|x| HashSet::from([x]))
        .collect();

    for (c0, c1, _) in connections {
        connect_circuit(&mut circuits, *c0, *c1);
        if circuits.len() == 1 {
            return (*c0, *c1);
        }
    }
    (0,0)
}


fn part_1(filename: &str, conn_num: usize, prod_num: i64) -> i64 {
    let boxes = parse_file(filename);
    let connections = find_closest_pairs(&boxes, conn_num as i64);
    let circuits = connect_circuit_list(&connections);
    let n = std::cmp::min(prod_num as usize, circuits.len());
    let product = circuits[0..n]
        .iter()
        .map(|x| x.len())
        .reduce(|prod, x| prod * x).unwrap() as i64;
    product
}

fn part_2(filename: &str) -> i64 {
    let boxes = parse_file(filename);
    let connections = find_closest_pairs(&boxes, -1);
    let (a, b) = connect_all_circuits(&connections, boxes.len());
    boxes[a].0 * boxes[b].0
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
    fn test_example1() {
        assert_eq!(part_1("inputs/example.txt", 10, 3), 40);
    }

    #[test]
    fn test_example2() {
        assert_eq!(part_2("inputs/example.txt"), 25272);
    }
}

fn main() {
    let part_1_solution = part_1("inputs/input.txt", 1000, 3);
    let part_2_solution = part_2("inputs/input.txt");
    println!("Part 1: {}", part_1_solution);
    println!("Part 2: {}", part_2_solution);
}
