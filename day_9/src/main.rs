use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
struct Point(i64, i64);
#[derive(Debug)]
struct Edge {
    parallel: i64,
    upper: i64,
    lower: i64,
    vertical: bool,
}

impl Edge {
    fn new(p1: &Point, p2: &Point) -> Self {
        if p1.0 == p2.0 {
            Edge {parallel: p1.0, 
                  upper: std::cmp::max(p1.1, p2.1), 
                  lower: std::cmp::min(p1.1, p2.1), 
                  vertical: true}
        } else if p1.1 == p2.1 {
            Edge {parallel: p1.1, 
                  upper: std::cmp::max(p1.0, p2.0), 
                  lower: std::cmp::min(p1.0, p2.0), 
                  vertical: false}
        } else {
            panic!("Invalid edge given p1 {:?} p2 {:?}", p1, p2);
        }
    }
}

#[derive(Debug)]
struct Shape {
    vertical_edges: Vec<Edge>,
    horizontal_edges: Vec<Edge>,
    vertices: Vec<Point>,
}

impl Shape {
    fn new() -> Self {
        Shape {vertical_edges: vec![], horizontal_edges: vec![], vertices: vec![]}
    }
    fn add_edge(&mut self, p1: &Point, p2: &Point) {
        let e = Edge::new(p1, p2);
        if e.vertical {
            self.vertical_edges.push(e);
            self.vertical_edges.sort_by_key(|x| x.parallel);
        } else {
            self.horizontal_edges.push(e);
            self.horizontal_edges.sort_by_key(|x| x.parallel);
        }
    }

    fn contains(&self, point: &Point) -> bool {
        // Ray-casting parity test using polygon vertices. Returns true if point is on an edge.
        let n = self.vertices.len();
        if n == 0 {
            return false;
        }

        let mut crossings = 0usize;
        for i in 0..n {
            let a = self.vertices[i];
            let b = self.vertices[(i + 1) % n];

            if a.0 == b.0 {
                // vertical edge
                let x = a.0;
                let y1 = std::cmp::min(a.1, b.1);
                let y2 = std::cmp::max(a.1, b.1);
                if point.1 >= y1 && point.1 <= y2 && point.0 == x {
                    return true; // on vertical edge
                }
                // count crossing if ray to +x intersects this edge; use half-open interval [y1, y2)
                if point.1 >= y1 && point.1 < y2 && x > point.0 {
                    crossings += 1;
                }
            } else if a.1 == b.1 {
                // horizontal edge
                let y = a.1;
                let x1 = std::cmp::min(a.0, b.0);
                let x2 = std::cmp::max(a.0, b.0);
                if point.1 == y && point.0 >= x1 && point.0 <= x2 {
                    return true; // on horizontal edge
                }
                // horizontal edges do not contribute to crossings
            } else {
                // should not happen for axis-aligned polygon
                continue;
            }
        }

        crossings % 2 == 1
    }
}

fn area(p1: &Point, p2: &Point) -> u64 {
    return (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1);
}

fn parse_line(line: &str) -> Point {
    let mut out = line.split(',').map(|x| x.trim().parse::<i64>().unwrap());
    Point(out.next().unwrap(), out.next().unwrap())
}

fn parse_file(filename: &str) -> Vec<Point> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let out: Vec<Point> = reader.lines()
        .map(|x| parse_line(&x.expect("Could not read line")))
        .collect();
    out
}

fn part_1(filename: &str) -> u64 {
    let points = parse_file(filename);
    let max_area = points.iter()
        .enumerate()
        .map(|(i,x)| points[i+1..].iter()
            .map(|y| area(x, y))
            .fold(0, |max, z| std::cmp::max(max, z)))
        .reduce(|max2, a| std::cmp::max(max2, a)).unwrap();
     max_area
}

fn make_shape(points: &Vec<Point>) -> Shape {
    let mut shape = Shape::new();
    for (p1, p2) in points[..(points.len()-1)].iter().zip(points[1..].iter()) {
        shape.add_edge(p1, p2);
    }
    shape.add_edge(&points[0], &points.last().unwrap());
    // store vertices for point-in-polygon tests
    shape.vertices = points.to_vec();
    shape
}

fn part_2(filename: &str) -> u64 {
    let points = parse_file(filename);
    let shape = make_shape(&points);
    let mut pairs: Vec<(&Point, &Point, u64)> = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for p2 in points[i+1..].iter().filter(|x| x.0 != p1.0 && x.1 != p1.1) {
            pairs.push((p1, p2, area(p1, p2)));
        }
    }
    pairs.sort_by_key(|x| std::cmp::Reverse(x.2));

    for (p1, p2, area) in pairs.iter() {
        let p12 = Point(p1.0, p2.1);
        let p21 = Point(p2.0, p1.1);
        // println!("Testing {:?}  {:?}  {}", p1, p2, area);
        if !shape.contains(&p12) || !shape.contains(&p21) {
            continue;
        }
        let mut contains = true;
        for i in std::cmp::min(p1.0, p2.0)..=std::cmp::max(p1.0, p2.0) {
            if !shape.contains(&Point(i, p1.1)) || !shape.contains(&Point(i, p2.1)) {
                contains = false;
                break;
            }
        }
        if !contains {
            continue;
        }
        contains = true;
        for j in std::cmp::min(p1.1, p2.1)..=std::cmp::max(p1.1, p2.1) {
            if !shape.contains(&Point(p1.0, j)) || !shape.contains(&Point(p2.0, j)) {
                contains = false;
                break;
            }
        }
        if contains {
            return *area;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(part_1("inputs/example.txt"), 50);
    }

    #[test]
    fn test_contains() {
        let points = parse_file("inputs/example.txt");
        let shape = make_shape(&points);
        assert!(!shape.contains(&Point(6,8)));
        assert!(shape.contains(&Point(8,4)));
        assert!(shape.contains(&Point(9,7)));
        assert!(shape.contains(&Point(2,4)));
        assert!(!shape.contains(&Point(6,2)));
        assert!(!shape.contains(&Point(12,3)));
        assert!(shape.contains(&Point(6,5)));
        assert!(!shape.contains(&Point(2,7)));
        assert!(shape.contains(&Point(10,6)));
    }

    #[test]
    fn test_example2() {
        assert_eq!(part_2("inputs/example.txt"), 24);
    }
}

fn main() {
    let part_1_area = part_1("inputs/input.txt");
    let part_2_area = part_2("inputs/input.txt");
    println!("Part 1 solution: {}", part_1_area);
    println!("Part 2 solution: {}", part_2_area);
}
